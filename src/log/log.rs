use bevy::prelude::*;

//Plugin
#[derive(Default)]
pub struct LogPlugin;

impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Log>();
    }
}

// Data
#[derive(Default)]
pub struct LogFragment {
    pub color: Color,
    pub text: String,
}

// Resources
#[derive(Default)]
pub struct Log(Vec<Vec<LogFragment>>);
