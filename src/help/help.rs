use std::fs;

use bevy::{prelude::*, app::AppExit, input::{ElementState, keyboard::KeyboardInput}, ecs::event::Events};
use bevy_ascii_terminal::Terminal;
use iyes_loopless::prelude::*;

use crate::{rendering::{EolAction, put_string_vec_formatted, LeftSize}, GameState, log::{Log, LogFragment}};

//Plugin
#[derive(Default)]
pub struct HelpPlugin;

impl Plugin for HelpPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<CurrentHelpPage>()
        .init_resource::<Events<HelpPageChangeEvent>>()
        .init_resource::<Events<HelpPageScrollEvent>>();
    }
}


// System
pub fn help_input (
    mut commands: Commands,

    keys: Res<Input<KeyCode>>,

    mut ev_key: EventReader<KeyboardInput>,
    mut ev_help_page_change: EventWriter<HelpPageChangeEvent>,
    mut ev_help_page_scroll: EventWriter<HelpPageScrollEvent>,
    //mut ev_restart: EventWriter<RestartEvent>,
) {
    for ev in ev_key.iter() {
        if ev.state == ElementState::Pressed {
            match ev.key_code {
                Some(KeyCode::Escape) => {
                    commands.insert_resource(NextState(GameState::Playing));
                }
                
                Some(KeyCode::Slash) => {
                    if keys.pressed(KeyCode::LShift) || keys.pressed(KeyCode::RShift) {
                        commands.insert_resource(NextState(GameState::Playing));
                    }
                }

                Some(KeyCode::Up) => {
                    ev_help_page_scroll.send(HelpPageScrollEvent(-1))
                }
                Some(KeyCode::Down) => {
                    ev_help_page_scroll.send(HelpPageScrollEvent(1))
                }
                
                Some(KeyCode::I) => {
                    ev_help_page_change.send(HelpPageChangeEvent(HelpPage::Intro));
                }
                Some(KeyCode::C) => {
                    ev_help_page_change.send(HelpPageChangeEvent(HelpPage::Controls));
                }
                Some(KeyCode::X) => {
                    ev_help_page_change.send(HelpPageChangeEvent(HelpPage::Combat));
                }
                Some(KeyCode::S) => {
                    ev_help_page_change.send(HelpPageChangeEvent(HelpPage::Stealth));
                }
                Some(KeyCode::T) => {
                    ev_help_page_change.send(HelpPageChangeEvent(HelpPage::Tips));
                }

                _ => {}
            }
        }
    }
}

pub fn start_help (
    mut ev_help_page_change: EventWriter<HelpPageChangeEvent>,
) {
    ev_help_page_change.send(HelpPageChangeEvent(HelpPage::Intro));
}

pub fn update_help_page (
    mut ev_help_page_change: EventReader<HelpPageChangeEvent>,

    mut current_help_page: ResMut<CurrentHelpPage>,

    mut terminal_query: Query<&mut Terminal>,
) {
    let mut terminal = terminal_query.single_mut();

    let term_size = terminal.size();

    let mut update_page = false;

    if let Some(ev) = ev_help_page_change.iter().next() {
        let page_text = fs::read_to_string(ev.path())
            .expect("Something went wrong reading the file.");

        let fragments = Log::string_to_lines_by_width(page_text, Color::WHITE, terminal.width() as usize + 1);

        current_help_page.contents = fragments;

        update_page = true;
    }

    if update_page {
        terminal.clear();

        let lines: &[Vec<LogFragment>];

        let line_count = current_help_page.contents.len();

        let mut current_line = (terminal.height() + 1) as i32;
        let mut current_length = 0;

        if line_count < terminal.height() as usize {
            lines = &current_help_page.contents[..];
        } else {
            lines = &current_help_page.contents[line_count-terminal.height() as usize..line_count]
        }

        for line in lines.iter() {
            current_line -= 1;
            [current_length, current_line] = put_string_vec_formatted([0, current_line], line, &mut terminal, EolAction::None);
        }
    }

}

// Data
#[derive(Clone)]
pub enum HelpPage {
    Intro,
    Controls,
    Combat,
    Stealth,
    Tips,

}
impl HelpPage {
    pub fn path(&self) -> &str {
        match self {
            HelpPage::Intro => "assets/help/intro.txt",
            HelpPage::Controls => "assets/help/controls.txt",
            HelpPage::Combat => "assets/help/combat.txt",
            HelpPage::Stealth => "assets/help/stealth.txt",
            HelpPage::Tips => "assets/help/tips.txt",
        }
    }
}

// Resource
#[derive(Clone)]
pub struct CurrentHelpPage{
    pub page: HelpPage,
    pub contents: Vec<Vec<LogFragment>>,
    pub scrolling: usize,
}
impl Default for CurrentHelpPage {
    fn default() -> Self {
        Self { page: HelpPage::Intro, contents: Default::default(), scrolling: Default::default() }
    }
}


// Events
#[derive(Clone, Deref, DerefMut)]
pub struct HelpPageChangeEvent(HelpPage);

// Events
#[derive(Clone, Deref, DerefMut)]
pub struct HelpPageScrollEvent(i32);