use bevy::prelude::*;
use bevy_ascii_terminal::Terminal;
use iyes_loopless::state::NextState;

use crate::GameState;

// Plugin
#[derive(Default)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<PlayerName>();
    }
}

// Systems
// This can probably become a generalized text input system later.
pub fn name_character (
    mut commands: Commands,

    mut char_evr: EventReader<ReceivedCharacter>,

    mut name: ResMut<PlayerName>,

    keys: Res<Input<KeyCode>>,

    mut terminal_query: Query<&mut Terminal>,
) {
    let mut terminal = terminal_query.single_mut();

    terminal.clear();

    let term_size = terminal.size();

    let mut display_text = String::from("Name: ");

    display_text.push_str(&*name);

    terminal.put_string([(term_size.x / 2) as i32 - (display_text.len() as i32 / 2), term_size.y as i32 / 2 ], &display_text);

    for ev in char_evr.iter() {
        match ev.char {
            '\n' => {}
            '\r' => {}
            '' => drop(name.pop()),
            '\u{0009}' => {}
            '\u{000b}' => {}


            _ => name.push(ev.char),
        }
        
        // Weird wrapping happens in gameplay if this is longer than 10.
        if name.len() > 10 {
            name.pop();
        }
    }

    if keys.just_pressed(KeyCode::Return) {
        commands.insert_resource(NextState(GameState::Restart));
    }
}

// Resources
#[derive(Deref, DerefMut, Clone)]
pub struct PlayerName(String);
impl Default for PlayerName {
    fn default() -> Self {
        Self(String::from("Cock"))
    }
}