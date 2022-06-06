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
    pub text: String,
    pub color: Color,
}
impl LogFragment {
    pub fn new(text: String, color: Color) -> Self {
        Self {text, color}
    }
}

// Resources
#[derive(Default)]
pub struct Log{
    pub lines: Vec<Vec<LogFragment>>
}
impl Log {
    pub fn fragment_string ( string: String, color: Color) -> Vec<LogFragment> {
        let strings: Vec<LogFragment> = string
            .split_inclusive(' ')
            .map(|s| LogFragment{text: s.to_string(), color: color} )
            .collect();
        strings
    }
    pub fn log_fragments ( &mut self, fragments: Vec<LogFragment> ) {
        self.lines.push(fragments);
    }

    pub fn log_string ( &mut self, string: String ) {
        self.lines.push(Log::fragment_string(string, Color::rgb(0.8, 0.8, 0.8)));
    }

    pub fn log_string_formatted ( &mut self, string: String, color: Color ) {
        self.lines.push(Log::fragment_string(string, color));
    }
}
