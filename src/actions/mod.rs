use bevy::prelude::{App, Plugin};

mod movement;
mod interactions;

#[derive(Default)]
pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_to_stage("pre_update", movement::update_collidables)
        .add_system_to_stage("post_update", movement::do_point_move);
    }
}