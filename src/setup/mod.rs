// Unglob later
use bevy::prelude::{App, Plugin};

mod setup;

#[derive(Default)]
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage("setup", setup::setup)
        .add_startup_system_to_stage("actor_placement", setup::setup_actors);
    }
}