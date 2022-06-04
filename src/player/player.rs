

use std::any::TypeId;

use bevy::input::{ElementState, keyboard::KeyboardInput};
use bevy::app::AppExit;
use bevy::prelude::*;

use crate::actions::healing::HealActionEvent;
use crate::actions::movement::PointMoveEvent;
use crate::actions::ranged::RangedAttackEvent;
use crate::rendering::window::WindowChangeEvent;

use self::targetting::StartTargetEvent;

use super::*;

pub mod ending;
pub mod targetting;


// Plugin
#[derive(Default)]
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<targetting::StartTargetEvent>()
        .init_resource::<targetting::Targetting>();
    }
}


// Components
#[derive(Component, Default, Copy, Clone)]
pub struct Player;

// Systems
/// Player input.
/// 
pub fn player_input_game (
    query: Query<(Entity, &Position), (With<Player>, With<TakesTurns>)>,

    mut ev_key: EventReader<KeyboardInput>,
    mut ev_movement: EventWriter<PointMoveEvent>,
    mut ev_heal: EventWriter<HealActionEvent>,
    mut ev_ranged_target: EventWriter<StartTargetEvent>,

    mut turns: ResMut<Turns>,
) {
    let ent = turns.order[turns.current];
    if let Ok((player, player_pos)) = query.get(ent) {
        for ev in ev_key.iter() {
            if ev.state == ElementState::Pressed {
                match ev.key_code {
                    // Cardinal Movement
                    Some(KeyCode::I) | Some(KeyCode::Numpad8) => {
                        ev_movement.send(PointMoveEvent{entity: player, movement: IVec2::new(0, 1)});
                        turns.progress_turn();
                    }
                    Some(KeyCode::Comma) | Some(KeyCode::Numpad2) => {
                        ev_movement.send(PointMoveEvent{entity: player, movement: IVec2::new(0, -1)});
                        turns.progress_turn();
                    }
                    Some(KeyCode::J) | Some(KeyCode::Numpad4) => {
                        ev_movement.send(PointMoveEvent{entity: player, movement: IVec2::new(-1, 0)});
                        turns.progress_turn();
                    }
                    Some(KeyCode::L) | Some(KeyCode::Numpad6) => {
                        ev_movement.send(PointMoveEvent{entity: player, movement: IVec2::new(1, 0)});
                        turns.progress_turn();
                    }
        
                    // Diagonal Movement
                    Some(KeyCode::U) | Some(KeyCode::Numpad7) => {
                        ev_movement.send(PointMoveEvent{entity: player, movement: IVec2::new(-1, 1)});
                        turns.progress_turn();
                    }
                    Some(KeyCode::O) | Some(KeyCode::Numpad9) => {
                        ev_movement.send(PointMoveEvent{entity: player, movement: IVec2::new(1, 1)});
                        turns.progress_turn();
                    }
                    Some(KeyCode::M) | Some(KeyCode::Numpad1) => {
                        ev_movement.send(PointMoveEvent{entity: player, movement: IVec2::new(-1, -1)});
                        turns.progress_turn();
                    }
                    Some(KeyCode::Period) | Some(KeyCode::Numpad3) => {
                        ev_movement.send(PointMoveEvent{entity: player, movement: IVec2::new(1, -1)});
                        turns.progress_turn();
                    }

                    // Do Nothing
                    Some(KeyCode::K) | Some(KeyCode::Numpad5) => {
                        turns.progress_turn();
                    }

                    // Heal
                    Some(KeyCode::V) => {
                        ev_heal.send(HealActionEvent{healing_entity: player});
                        turns.progress_turn();
                    }

                    Some(KeyCode::C) => {
                        ev_ranged_target.send(StartTargetEvent::new(TypeId::of::<RangedAttackEvent>(), **player_pos))
                    }

                    _ => {}
                }
            }
        }
    }
}

pub fn player_input_meta (
    mut commands: Commands,

    keys: Res<Input<KeyCode>>,

    mut ev_key: EventReader<KeyboardInput>,
    mut ev_exit: EventWriter<AppExit>,
    mut ev_window_change: EventWriter<WindowChangeEvent>,
    //mut ev_restart: EventWriter<RestartEvent>,
) {
    for ev in ev_key.iter() {
        if ev.state == ElementState::Pressed {
            match ev.key_code {
                Some(KeyCode::Escape) => {
                    ev_exit.send(AppExit);
                }
                Some(KeyCode::NumpadAdd) | Some(KeyCode::Equals) => {
                    ev_window_change.send(WindowChangeEvent(1));
                }
                Some(KeyCode::NumpadSubtract) | Some(KeyCode::Minus) => {
                    ev_window_change.send(WindowChangeEvent(-1));
                }
                Some(KeyCode::R) => {
                    if keys.pressed(KeyCode::LShift) || keys.pressed(KeyCode::RShift) {
                        commands.insert_resource(NextState(GameState::Restart));
                    }
                }

                _ => {}
            }
        }
    }
}


