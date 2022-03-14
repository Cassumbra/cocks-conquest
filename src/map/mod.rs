use bevy::prelude::{App, Plugin};
use super::*;

mod map;

#[derive(Default)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage("map_gen", map::entity_map_rooms_passages);
    }
}