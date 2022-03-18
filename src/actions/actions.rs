use bevy::prelude::*;

pub mod movement;
mod interactions;

#[derive(Default)]
pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<movement::CollidableChangeEvent>()
        .add_event::<movement::PointMoveEvent>();
    }
}