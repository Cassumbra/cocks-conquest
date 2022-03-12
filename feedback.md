## Project structure

Currently the project is separated by:
* Components
* Systems
* events
* resources

Recent bevy projects tended to use `Plugin`s to separate unrelated code. Most
rust modules (files) in a bevy project define and export a single `Plugin` that
gets added in the `main.rs` to the `app`.

In this game, I would have had a `ai.rs` that defines all components, resources
and systems related to ai, and only make the `AiDriven` component `pub` this
way I can add it to enemy that might be spawned in other systems.

I would also have a `map_gen.rs` module etc.

## Random thoughts

### AI

If the AI behaviors are mutually exclusive, you probably want to use an `enum`
for it instead of multiple `struct`s.

```rust
// Instead of:
#[derive(Component, Default, Copy, Clone)]
pub struct AIDoNothing;

#[derive(Component, Default, Copy, Clone)]
pub struct AIWalkDown;

#[derive(Component, Default, Copy, Clone)]
pub struct AIRandMove;

// You would have:
#[derive(Component, Default, Copy, Clone)]
pub enum AiDriven {
  RandMove,
  WalkDown,
  DoNothing,
}
```
This way you can't have an entity that is both `AIDoNothing` and `AIWalkDown`
(which is contradictory). If later-on, you want multiple AI behaviors, well
let's wait until then to think about it.

### Startup Systems

You can either use `SystemSet::on_enter(GameState::State).with_system(oneshot_system)`
or `app.add_startup_system(oneshot_system)` to run a system only once. This is
relevant for your `entity_map_rooms_passages` (that could be renamed
`generate_map`).

### Game state

On that note. Instead of having a
```rust
pub fn entity_map_rooms_passages (
    mut has_run: Local<bool>,
    mut game_state: ResMut<State<GameState>>,
    // ...
) {
    if *has_run {
        game_state.set(GameState::PlayerPlacement).unwrap();
        return;
    }
    // setup code
    // ...
    *has_run = true;
}
// do this:
pub fn entity_map_rooms_passages (
    mut game_state: ResMut<State<GameState>>,
    // ...
) {
    // setup code
    // ...
    game_state.set(GameState::PlayerPlacement).unwrap();
}
```

Otherwise, next time you enter `GameState::MapGen` the system won't run!


### `fill_rect`

I would have defined `fill_rect` in `systems/map.rs` as follow:

```rust
fn fill_rect ( commands: &mut Commands, map_objects: &mut Grid<Option<Entity>>, bundle: impl Bundle + Copy, rect: &Rectangle) {
    let rect_bounds = [rect.pos1.x, rect.pos1.y]..=[rect.pos2.x, rect.pos2.y];
    for (pos, opt_entity) in map_objects.clone().rect_iter(rect_bounds) {
        if let Some(entity) = opt_entity {
            commands.entity(*entity).despawn();
        }
        let entity = commands.spawn_bundle(bundle)
            .insert(Position(pos))
            .id();
        map_objects[pos] = Some(entity);
    }
}
```

A few notes:
* You can use `(pos, opt_entity)` in the `for` binding so that it's easier to
  separate the entity from the position
* You don't need to extract the old entity from the original `map_objects`,
  `opt_entity` should be exactly the same value
* due to https://docs.rs/sark_grids/latest/sark_grids/grid/struct.Grid.html#impl-Index%3CP%3E
  you can use a `IVec2` to index a `Grid`

You want to replace:
```rust
let something = some.complex.operations();
if something.is_some() {
  let old_something = some.complex.operations();
  do_something(old_something);
}
```
By:
```rust
let opt_something = some.complex.operations();
if let Some(something) = opt_something {
  do_something(something);
}
```
I saw a few places in the code where that's possible.

### `IsTurn`

Problem: Adding and removing components is _very resource intensive_ (because
bevy has to update many different tables when you do this). So instead of adding
and removing the `IsTurn` component once per actor, you probably want something
else.

People in the bevy community have difficult coming with a good system for
dynamically sized turn based game. I've no idea how to solve this, I won't be
able to provide a solution to something multiple very smart people thought
about for a while.

You could get rid of all the `IsTurn` and related components, create a
`GameState::ResolveTurn` state and use system labels to make sure the
turn resolution related systems run in the correct order.

See: https://bevy-cheatbook.github.io/programming/labels.html

### Movement

Instead of adding a `Movement` component, to an entity, have a `Movement` event
and your `do_movements` handle those:
```rust
enum Move {
    Teleport(Entity, Position),
    Relative(Entity, IVec2),
}
pub fn do_movements(
    mut move_events: EventReader<Move>,
    mut positions: Query<&mut Position>,
    collisions: CollisionDetector,
) {
    // TODO: update position grid mapping as well
    for event in move_events.iter() {
        match event {
            Move::Teleport(entity, new_position) => {
                let mut old_position = positions.get(entity).unwrap();
                *old_position = new_position;
            }
            Move::Relative(entity, direction) => {
                let mut old_position = positions.get(entity).unwrap();
                let new_position = Position(old_position.0 + direction);
                if collisions.can_move(old_position, new_position) {
                    *old_position = new_position;
                }
            }
        }
    }
}
```

### Collisions

Looks like your `update_collidables_new` is not added properly to the ECS, so
it won't receive your `CollidableChangeEvent`s.

Here is my suggestion: you could make collisions much more bearable with a
`SystemParam`:

See: https://docs.rs/bevy/0.6.1/bevy/ecs/system/trait.SystemParam.html

```rust
use bevy::ecs::system::SystemParam;

enum CollisionLayer {
    Walking,
    Flying,
    Swimming,
    // etc.
}
#[derive(SystemParam)]
pub struct CollisionDetector<'w, 's> {
    grid: Res<'w, EntityGrid>,
    layers: Query<'w, 's, &'static CollisionLayer>,
}
impl<'w, 's> CollisionDetector<'w, 's> {
    pub fn can_move(&self, from: Position, to: Position) -> bool {
        if let Some(moving_entity) = self.grid.at(from).unwrap();
        let moving_layer = self.layers.get(moving_entity).unwrap();
        // check collision layer of all tiles in the way (comparing it to that
        // of the entity in the `from` position)
    }
}
```

### Grid

You want to keep a `Position -> Entity` map somewhere. Either with a
`HashMap<IVec2, Entity>` or `Grid<Option<Entity>>`. And should be keeping
up-to-date all the time. Currently you are using `Collidables` as that map. I
suggest you make the inner field private.

```rust
pub struct EntityGrid(HashMap<IVec2, Entity>);
impl EntityGrid {
    pub fn at(&self, position: Position) -> Option<Entity> {
        self.0.get(position.0)
    }
    pub fn add_entity(&mut self, entity: Entity, at: Position) -> bool {
        // Add entity if there is none on the same place yet. (or maybe
        // you want to be able to have multiple entities at the same
        // tile?)
    }
    pub fn move_entity(&mut self, from: Position, to: Position) -> bool {
        // etc.
    }
}
```

Avoid making the `HashMap` (or the `Grid<Option<>>` if you chose that option)
public. Afterward, just look for places where you use `add_entity` or
`move_entity` if there is a problem.

### `bevy_inspector_egui`

This is a very useful tool for debugging and understanding what is going on in
the ECS. Try using it! https://github.com/jakobhellermann/bevy-inspector-egui .

For example:
```rust
#[cfg(debug_assertions)]
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

#[cfg_attr(debug_assertions, derive(Inspectable))]
struct Position(IVec2);

// in the plugin definition or `main.rs`:
#[cfg(debug_assertions)]
app
    .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
    .register_inspectable::<Position>();
```

Now there should be a floating window in your game that shows you the state of
all the components in your game!


### Input

You should check Alice's [leafwig input manager](https://github.com/Leafwing-Studios/leafwing_input_manager/).
To handle player input.

### Window and Rendering

Now I've already spent more than 2 hours on this. And I would need to review
the `bevy_ascii_terminal` documentation as well.
