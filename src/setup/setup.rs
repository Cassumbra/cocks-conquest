// Unglob later
use bevy::prelude::*;
use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;
use sark_grids::Grid;
use super::*;




pub fn setup (
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
}