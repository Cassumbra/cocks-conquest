// Unglob later
use bevy::prelude::*;
use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;
use sark_grids::Grid;
use crate::actions::movement::Collidables;
use crate::rendering::window::WindowChangeEvent;

use super::*;

// Systems
pub fn setup (
    mut commands: Commands,

    map_size: Res<MapSize>,

    bottom_size: Res<BottomSize>,
    left_size: Res<LeftSize>,

    mut ev_window_change: EventWriter<WindowChangeEvent>,
) {
    ev_window_change.send(WindowChangeEvent(1));

    let size = [map_size.width + left_size.width, map_size.height + bottom_size.height];

    let term_bundle = TerminalBundle::new().with_size(size);

    commands.spawn_bundle(term_bundle)
            .insert(Indestructible);

    commands.spawn_bundle(TiledCameraBundle::new()
            .with_tile_count(size))
            .insert(Indestructible);

    commands.insert_resource(NextState(GameState::PickName));
}

pub fn restart (
    mut commands: Commands,

    map_size: Res<MapSize>,
    bottom_size: Res<BottomSize>,
    left_size: Res<LeftSize>,

    query: Query<Entity, Without<Indestructible>>,
) {
    for ent in query.iter() {
        commands.entity(ent).despawn();
    }

    let size = [map_size.width + left_size.width, map_size.height + bottom_size.height];

    let collidables: Grid<Option<Entity>> = Grid::default([map_size.width, map_size.height]);
    commands.insert_resource(Collidables(collidables));

    commands.insert_resource(TemporaryTerminal(Terminal::with_size(size)));

    commands.insert_resource(Log{
        lines: vec![
        Log::fragment_string(" Welcome to Cock's Conquest!  \n Press shift+/ (?) for help.  \n You can restart with shift+r at any time.".to_string(), Color::CYAN),
        ]
    });

    commands.insert_resource(NextState(GameState::MapGen));
}

// Components
#[derive(Component)]
pub struct Indestructible;