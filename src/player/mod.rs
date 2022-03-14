// Unglob later
use bevy::prelude::{App, Plugin};

mod player_input;

#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_to_stage("update", player_input::player_input);
    }
}