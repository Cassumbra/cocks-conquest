// Unglob later
use bevy::prelude::{App, Plugin};
use super::*;

mod ai;

#[derive(Default)]
pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_to_stage("update", ai::walk_at_player)
        .add_system_to_stage("update", ai::generic_brain);
    }
}