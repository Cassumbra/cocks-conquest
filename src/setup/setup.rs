// Unglob later
use bevy::prelude::*;
use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;
use sark_grids::Grid;
use crate::{actions::movement::Collidables, rendering::window::WindowChangeEvent};

use super::*;

pub fn setup (
    mut commands: Commands,

    map_size: Res<MapSize>,
    bottom_size: Res<BottomSize>,

    mut ev_window_change: EventWriter<WindowChangeEvent>,
) {
    let size = [map_size.width, map_size.height + bottom_size.height];

    let mut term_bundle = TerminalBundle::new().with_size(size);
    let terminal = &mut term_bundle.terminal;

    commands.spawn_bundle(term_bundle);

    commands.spawn_bundle(TiledCameraBundle::new()
        .with_tile_count(size));

    let collidables: Grid<Option<Entity>> = Grid::default([map_size.width, map_size.height]);
    commands.insert_resource(Collidables(collidables));

    commands.insert_resource(TemporaryTerminal(Terminal::with_size(size)));

    ev_window_change.send(WindowChangeEvent(1))
}