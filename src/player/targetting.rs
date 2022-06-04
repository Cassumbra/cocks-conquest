use bevy::{prelude::*, input::{keyboard::KeyboardInput, ElementState}};
use iyes_loopless::state::NextState;

use std::{any::TypeId};

use crate::{GameState, map::MapSize, actions::ranged::RangedAttackEvent};

// Events
pub struct StartTargetEvent{
    pub intent: TypeId,
    pub position: IVec2,
}
impl StartTargetEvent {
    pub fn new(intent: TypeId, position: IVec2) -> Self {
        Self {intent, position }
    }
}

//pub struct FinishTargetEvent<T>(T);

// Resources
pub struct Targetting {
    pub intent: TypeId,
    pub position: IVec2,
    pub target: IVec2,
}
impl Default for Targetting {
    fn default() -> Self {
        Self { intent: TypeId::of::<RangedAttackEvent>(), position: Default::default(), target: Default::default() }
    }
}

// Systems
pub fn start_targetting (
    mut commands: Commands,

    mut ev_start_targetting: EventReader<StartTargetEvent>,
    
    mut targetting: ResMut<Targetting>,
) {
    if let Some(ev) = ev_start_targetting.iter().next() {
        targetting.intent = ev.intent;
        targetting.position = ev.position;
        targetting.target = ev.position;
        commands.insert_resource(NextState(GameState::Targetting));
    }
}

pub fn targetting (
    mut commands: Commands,

    mut ev_key: EventReader<KeyboardInput>,

    map_size: Res<MapSize>,
    mut targetting: ResMut<Targetting>,
    //bottom_size: Res<BottomSize>,
) {
    for ev in ev_key.iter() {
        if ev.state == ElementState::Pressed {
            match ev.key_code {
                // Cardinal Movement
                Some(KeyCode::I) | Some(KeyCode::Numpad8) => {
                    targetting.target += IVec2::new(0, 1);
                }
                Some(KeyCode::Comma) | Some(KeyCode::Numpad2) => {
                    targetting.target += IVec2::new(0, -1);
                }
                Some(KeyCode::J) | Some(KeyCode::Numpad4) => {
                    targetting.target += IVec2::new(-1, 0);
                }
                Some(KeyCode::L) | Some(KeyCode::Numpad6) => {
                    targetting.target += IVec2::new(1, 0);
                }
    
                // Diagonal Movement
                Some(KeyCode::U) | Some(KeyCode::Numpad7) => {
                    targetting.target += IVec2::new(-1, 1);
                }
                Some(KeyCode::O) | Some(KeyCode::Numpad9) => {
                    targetting.target += IVec2::new(1, 1);
                }
                Some(KeyCode::M) | Some(KeyCode::Numpad1) => {
                    targetting.target += IVec2::new(-1, -1);
                }
                Some(KeyCode::Period) | Some(KeyCode::Numpad3) => {
                    targetting.target += IVec2::new(1, -1);
                }

                // Select Target
                Some(KeyCode::Return) | Some(KeyCode::Space) | Some(KeyCode::C) => {
                    // TODO:
                    commands.insert_resource(NextState(GameState::Playing));
                }

                // Cancel Targetting
                Some(KeyCode::Escape) => {
                    commands.insert_resource(NextState(GameState::Playing));
                }

                _ => {}
            }
        }
    }

    targetting.target.x.clamp(0, map_size.width as i32 - 1);
    targetting.target.y.clamp(0, map_size.height as i32 - 1);
}