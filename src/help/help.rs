use std::fs;

use bevy::{prelude::*, app::AppExit, input::{ElementState, keyboard::KeyboardInput}, ecs::event::Events};
use iyes_loopless::prelude::*;

use crate::{rendering::window::WindowChangeEvent, GameState};

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

                }
                Some(KeyCode::Down) => {
                    
                }
                
                Some(KeyCode::I) => {
                    println!("A");
                    ev_help_page_change.send(HelpPageChangeEvent{page: HelpPage::Intro});
                }

                _ => {}
            }
        }
    }
}

pub fn update_help_page (
    mut ev_help_page_change: EventReader<HelpPageChangeEvent>,
) {
    if let Some(ev) = ev_help_page_change.iter().next() {
        let page_text = fs::read_to_string(ev.page.path())
            .expect("Something went wrong reading the file.");

        println!("{}", page_text);
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
    pub contents: String,
    pub scrolling: usize,
}
impl Default for CurrentHelpPage {
    fn default() -> Self {
        Self { page: HelpPage::Intro, contents: Default::default(), scrolling: Default::default() }
    }
}


// Events
#[derive(Clone)]
pub struct HelpPageChangeEvent {
    pub page: HelpPage,
}
