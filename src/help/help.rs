use std::fs;

use bevy::{prelude::*, app::AppExit, input::{ElementState, keyboard::KeyboardInput}, ecs::event::Events};
use bevy_ascii_terminal::Terminal;
use iyes_loopless::prelude::*;

use crate::{rendering::{window::WindowChangeEvent, put_string_vec_formatted}, GameState, log::{Log, LogFragment}};

//Plugin
#[derive(Default)]
pub struct HelpPlugin;

impl Plugin for HelpPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<CurrentHelpPage>()
        .init_resource::<Events<HelpPageChangeEvent>>();
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

                _ => {}
            }
        }
    }
}

pub fn update_help_page (
    mut ev_help_page_change: EventReader<HelpPageChangeEvent>,

    mut current_help_page: ResMut<CurrentHelpPage>,

    mut terminal_query: Query<&mut Terminal>,
) {
    let mut update_page = false;

    if let Some(ev) = ev_help_page_change.iter().next() {
        let page_text = fs::read_to_string(ev.path())
            .expect("Something went wrong reading the file.");

        let fragments = Log::fragment_string(page_text, Color::WHITE);

        current_help_page.contents = fragments;

        update_page = true;
    }

    if update_page {
        let mut terminal = terminal_query.single_mut();

        let lines: &[Vec<LogFragment>];

        if log.lines.len() < bottom_size.height as usize {
            lines = &log.lines[..];
        } else {
            lines = &log.lines[log.lines.len()-bottom_size.height as usize..log.lines.len()]
        }

        for line in lines.iter().rev() {
            current_line -= 1;
            [current_length, current_line] = put_string_vec_formatted([(left_size.width-1) as i32, current_line], line, &mut terminal.0, EolAction::None);
        }
    }

}

// Data
#[derive(Clone)]
pub enum HelpPage {
    Intro,
    Controls,
    Stealth,
    Tips,

}
impl HelpPage {
    pub fn path(&self) -> &str {
        match self {
            HelpPage::Intro => "assets/help/intro.txt",
            HelpPage::Controls => todo!(),
            HelpPage::Stealth => todo!(),
            HelpPage::Tips => todo!(),
        }
    }
}

// Resource
#[derive(Clone)]
pub struct CurrentHelpPage{
    pub page: HelpPage,
    pub contents: Vec<LogFragment>,
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