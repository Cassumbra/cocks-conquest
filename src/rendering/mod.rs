// Unglob later
use bevy::prelude::{App, Plugin};

mod rendering;
pub mod window;

#[derive(Default)]
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_to_stage("pre_update", rendering::update_render_order)
        .add_system_to_stage("update", rendering::render);
    }
}

