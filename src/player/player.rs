

use std::any::TypeId;

use bevy::input::{ButtonState, keyboard::KeyboardInput};
use bevy::app::AppExit;
use bevy::prelude::*;

use crate::actions::healing::HealActionEvent;
use crate::actions::movement::PointMoveEvent;
use crate::actions::ranged::{RangedAttackEvent, RangedAttacker};
use crate::actors::stats::{StatModification, StatType, Operation};
use crate::actors::status_effects::{StatusEffectEvent, StatusEffect, StatusEffectType, StatusEffectStacking, StatusEffectApplication};
use crate::rendering::window::WindowChangeEvent;

use self::targetting::{StartTargetEvent, TargetIntent, FinishTargetEvent};

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
        .add_event::<targetting::FinishTargetEvent>()
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
    // Query will be fucky if the player is ever an actor without ranged attacks. Later, we may want to move the ranged attacker part to another system. Or just change how we do it here.
    query: Query<(Entity, &Position, &RangedAttacker), (With<Player>, With<TakesTurns>)>,

    mut ev_key: EventReader<KeyboardInput>,
    mut ev_movement: EventWriter<PointMoveEvent>,
    mut ev_status_effect: EventWriter<StatusEffectEvent>,
    mut ev_heal: EventWriter<HealActionEvent>,
    mut ev_target: EventWriter<StartTargetEvent>,

    mut turns: ResMut<Turns>,
) {
    let ent = turns.order[turns.current];
    if let Ok((player, player_pos, ranged)) = query.get(ent) {
        for ev in ev_key.iter() {
            if ev.state == ButtonState::Pressed {
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
                        // TODO: Maybe this should be an effect for anything that doesn't move for a turn instead of just the player?
                        ev_status_effect.send(StatusEffectEvent{
                            application: StatusEffectApplication {
                                effect: StatusEffect {
                                    status_type: StatusEffectType::Sneaking,
                                    from: Some(player),
                                    tile_modification: None,
                                    duration: Some(2),
                                    stat_modification: Some(StatModification{stat_type: StatType::StealthRange, operation: Operation::DivideRound(2)})
                                },
                                stacking: StatusEffectStacking::Refreshes,
                            },

                            entity: player,
                        });
                        turns.progress_turn();
                    }

                    // Heal
                    Some(KeyCode::V) => {
                        ev_heal.send(HealActionEvent{healing_entity: player});
                        turns.progress_turn();
                    }

                    Some(KeyCode::C) => {
                        // this is UGLY
                        ev_target.send(StartTargetEvent::new(TargetIntent::RangedAttack(RangedAttackEvent{targetting_entity: player, target: **player_pos, projectile: ranged.projectiles[0].clone() }) , **player_pos))
                    }

                    _ => {}
                }
            }
        }
    }
}

pub fn player_receive_targetting (
    mut ev_ranged_attack: EventWriter<RangedAttackEvent>,
    mut ev_finish_target: EventReader<FinishTargetEvent>,

    mut turns: ResMut<Turns>,
) {
    for ev in ev_finish_target.iter() {
        match &ev.intent {
            TargetIntent::RangedAttack(attack) => {

                ev_ranged_attack.send(attack.clone());
    
                turns.progress_turn();
            }
    
            _ => {
    
            }
        }
    }

    
}

pub fn player_input_meta_general (
    mut commands: Commands,

    keys: Res<Input<KeyCode>>,

    mut ev_key: EventReader<KeyboardInput>,
    mut ev_exit: EventWriter<AppExit>,
    mut ev_window_change: EventWriter<WindowChangeEvent>,
    //mut ev_restart: EventWriter<RestartEvent>,
) {
    for ev in ev_key.iter() {
        if ev.state == ButtonState::Pressed {
            match ev.key_code {
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

pub fn player_input_meta_playing (
    mut commands: Commands,

    keys: Res<Input<KeyCode>>,

    mut ev_char: EventReader<ReceivedCharacter>,
    mut ev_key: EventReader<KeyboardInput>,
    mut ev_exit: EventWriter<AppExit>,
    mut ev_window_change: EventWriter<WindowChangeEvent>,
    //mut ev_restart: EventWriter<RestartEvent>,
) {
    for ev in ev_char.iter() {
        match ev.char {
            '?' => commands.insert_resource(NextState(GameState::Help)),
        
            _ => {}
        }
    }

    for ev in ev_key.iter() {
        if ev.state == ButtonState::Pressed {
            match ev.key_code {
                Some(KeyCode::Escape) => {
                    ev_exit.send(AppExit);
                }

                _ => {}
            }
        }
    }
}


