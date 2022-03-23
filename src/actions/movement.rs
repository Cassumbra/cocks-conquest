// Unglob later
use bevy::prelude::*;
use sark_grids::grid::Grid;
use adam_fov_rs::{VisibilityMap, fov};
use super::{super::*, interactions::BumpEvent};


//Plugin
#[derive(Default)]
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<PointMoveEvent>()
        .add_event::<CollidableChangeEvent>()
        .init_resource::<Collidables>();
    }
}

// Events
// We may make a "LineMoveEvent" later.
pub struct PointMoveEvent {
    pub entity: Entity,
    pub movement: IVec2,
}

pub struct CollidableChangeEvent {
    pub old_position: IVec2,
    pub new_position: IVec2,
    pub entity: Entity,
}

// Resources
#[derive(Default, Clone)]
pub struct Collidables(pub Grid<Option<Entity>>);


// Systems
pub fn do_point_move(
    mut ev_collidable_change: EventWriter<CollidableChangeEvent>,
    mut ev_bump_event: EventWriter<BumpEvent>,
    mut ev_movement_event: EventReader<PointMoveEvent>,

    mut query: Query<(&mut Position, Option<&Collides>, Option<&Name>)>,
    map_size: Res<MapSize>,
    collidables: Res<Collidables>,
) {
    for ev in ev_movement_event.iter() {
        if let Ok((mut pos, col, opt_name)) = query.get_mut(ev.entity) {
            let new_pos = pos.0 + ev.movement;

            if let Some(collidable_entity) = collidables.0[[new_pos.x as u32, new_pos.y as u32]] {
                if col.is_some() {
                    // When we remake our game, we should create some logs.
                    // We need a log that is shown to the player, and we need a debug log.
                    // An error log may be good too.
                    // Error and debug may be combined?
                    if let Some(name) = opt_name {
                        println!("{} bonked at {:?}.", name.as_str(), pos.0)
                    }
                    else {
                        println!("{:?} bonked at {:?}.", ev.entity, pos.0)
                    }
                    ev_bump_event.send(BumpEvent{bumping_entity: ev.entity, bumped_entity: collidable_entity})
                }
                
            }
            else if new_pos.x >= map_size.width as i32 || new_pos.y >= map_size.height as i32 || new_pos.x <= -1 || new_pos.y <= -1 {
                // Nothing for us to bonk. We are simply attempting to walk into the infinity, which is not allowed (at this moment)
            }
            else {
                let old_pos = pos.0;
                pos.0 = new_pos;
                ev_collidable_change.send(CollidableChangeEvent{
                    old_position: old_pos,
                    new_position: new_pos,
                    entity: ev.entity,
                });
            }
        }
    }
}

pub fn update_collidables( 
    mut ev_collidable_change: EventReader<CollidableChangeEvent>,
    query: Query<(Entity, &Position), (With<Collides>, Added<Collides>)>,
    mut collidables: ResMut<Collidables>,
) {
    for (ent, pos) in query.iter() {
        collidables.0[[pos.0.x as u32, pos.0.y as u32]] = Some(ent);
    }
    for ev in ev_collidable_change.iter() {
        //println!("Collidable update");
        collidables.0[[ev.old_position.x as u32, ev.old_position.y as u32]] = None;
        collidables.0[[ev.new_position.x as u32, ev.new_position.y as u32]] = Some(ev.entity);
    }
}