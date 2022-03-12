// Unglob later
use bevy::prelude::*;
use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;
use sark_grids::Grid;
use super::super::*;

pub fn setup(
    mut game_state: ResMut<State<GameState>>, 
    mut commands: Commands,
    map_size: Res<MapSize>,
    mut bottom_size: Res<BottomSize>,
) {
    let size = [map_size.width, map_size.height + bottom_size.height];

    let mut term_bundle = TerminalBundle::new().with_size(size);
    let terminal = &mut term_bundle.terminal;

    commands.spawn_bundle(term_bundle);

    commands.spawn_bundle(TiledCameraBundle::new()
        .with_tile_count(size));

    let collidables: Grid<Option<Entity>> = Grid::default([map_size.width, map_size.height]);
    commands.insert_resource(Collidables(collidables));
    
    game_state.set(GameState::MapGen).unwrap();
}

pub fn setup_actors(
    mut game_state: ResMut<State<GameState>>,
    //mut run_state: ResMut<State<RunState>>,
    mut commands: Commands,
    rooms: Res<Rooms>,
) {
    let mut other_rooms = rooms.0.clone();
    let room_first = other_rooms.swap_remove(0);

    commands.spawn()
        .insert_bundle(actors::PlayerBundle{..Default::default()})
        .insert(Position(room_first.center()))
        .insert(Name::new("Cass Cock"));

    for (i, room) in other_rooms.iter().enumerate() {
        commands.spawn()
            .insert_bundle(actors::SoldierBundle{..Default::default()})
            .insert(Position(room.center()))
            .insert(Name::new(format!("Soldier {}", i)));
    }

    //game_state.set(GameState::Playing).unwrap();
    //run_state.set(RunState::Ready).unwrap();
}