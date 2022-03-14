// Unglob later
use bevy::prelude::*;
use super::*;

mod turn;

#[derive(Default)]
pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_to_stage("pre_update", turn::ensure_turn_exists)
        .add_system_to_stage("pre_update", turn::update_turn_order)
        .add_system_to_stage("pre_update", turn::update_turn);
    }
}