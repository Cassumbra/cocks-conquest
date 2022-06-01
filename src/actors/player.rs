

use bevy::input::{ElementState, keyboard::KeyboardInput};
use bevy::app::AppExit;
use bevy::prelude::*;

use crate::actions::healing::{CanHeal, HealActionEvent};
use crate::actions::attack::{Attack, Dice};
use crate::actions::movement::PointMoveEvent;
use crate::actions::vore::DoesVore;
use crate::data::{Position, Collides};
use crate::log::Log;
use crate::rendering::Renderable;
use crate::rendering::window::WindowChangeEvent;
use crate::turn::Turns;

use super::*;



// Components
#[derive(Component, Default, Copy, Clone)]
pub struct Player;

// Bundles
#[derive(Bundle, Clone)]
pub struct PlayerBundle {
    pub position: Position,
    pub renderable: Renderable,
    pub collides: Collides,
    pub player: Player,
    pub takes_turns: TakesTurns,
    pub vision: Vision,
    pub mind_map: MindMap,
    pub stats: Stats,
    pub fatal_stats: FatalStats,
    pub relations: Relations,
    pub melee_attacker: MeleeAttacker,
    pub does_vore: DoesVore,
    pub can_heal: CanHeal,
}
impl Default for PlayerBundle {
    fn default() -> PlayerBundle {
        PlayerBundle {
            position: Position (IVec2::new(0, 0)),
            renderable: Renderable {
                tile: Tile {
                    glyph: '@',
                    fg_color: Color::RED,
                    bg_color: Color::NONE,
                },
                order: 128
            },
            collides: Collides,
            player: Player,
            takes_turns: TakesTurns,
            vision: Vision{..Default::default()},
            mind_map: MindMap{..Default::default()},
            stats: Stats(
                BTreeMap::from([
                    (StatType::Health, Stat::new(0, 7)),
                    (StatType::CumPoints, Stat::with_value(15, 0, 60)),
                ])
            ),
            fatal_stats: FatalStats{..Default::default()},
            relations: Relations::new(vec![Alignment::Cock], vec![Alignment::Cock], vec![Alignment::AntiCock]),
            melee_attacker: MeleeAttacker{attacks: vec![
                Attack{
                    interact_text: vec!["{attacker} breathes their stink into {attacked}'s head, lowering their resistance by {amount}!".to_string(),
                                        "{attacker} gives {attacked} a big smelly kiss with their cockmaw, lowering their resistance by {amount}!".to_string(),],
                    damage: Dice::new("1d4 * -1"),
                    damage_type: StatType::Resistance,
                    cost: Dice::new("0"),
                    cost_type: StatType::Health,

                    // Temporary for now
                    ..default()
                }
            ]},
            does_vore: DoesVore,
            can_heal: CanHeal,
        }
    }
}

// Systems
/// Player input.
/// 
pub fn player_input_game (
    query: Query<Entity, (With<Position>, With<Player>, With<TakesTurns>)>,

    mut ev_key: EventReader<KeyboardInput>,
    mut ev_movement_event: EventWriter<PointMoveEvent>,
    mut ev_heal_event: EventWriter<HealActionEvent>,

    mut turns: ResMut<Turns>,
) {
    let ent = turns.order[turns.current];
    if let Ok(player) = query.get(ent) {
        for ev in ev_key.iter() {
            if ev.state == ElementState::Pressed {
                match ev.key_code {
                    // Cardinal Movement
                    Some(KeyCode::I) | Some(KeyCode::Numpad8) => {
                        ev_movement_event.send(PointMoveEvent{entity: player, movement: IVec2::new(0, 1)});
                        turns.progress_turn();
                    }
                    Some(KeyCode::Comma) | Some(KeyCode::Numpad2) => {
                        ev_movement_event.send(PointMoveEvent{entity: player, movement: IVec2::new(0, -1)});
                        turns.progress_turn();
                    }
                    Some(KeyCode::J) | Some(KeyCode::Numpad4) => {
                        ev_movement_event.send(PointMoveEvent{entity: player, movement: IVec2::new(-1, 0)});
                        turns.progress_turn();
                    }
                    Some(KeyCode::L) | Some(KeyCode::Numpad6) => {
                        ev_movement_event.send(PointMoveEvent{entity: player, movement: IVec2::new(1, 0)});
                        turns.progress_turn();
                    }
        
                    // Diagonal Movement
                    Some(KeyCode::U) | Some(KeyCode::Numpad7) => {
                        ev_movement_event.send(PointMoveEvent{entity: player, movement: IVec2::new(-1, 1)});
                        turns.progress_turn();
                    }
                    Some(KeyCode::O) | Some(KeyCode::Numpad9) => {
                        ev_movement_event.send(PointMoveEvent{entity: player, movement: IVec2::new(1, 1)});
                        turns.progress_turn();
                    }
                    Some(KeyCode::M) | Some(KeyCode::Numpad1) => {
                        ev_movement_event.send(PointMoveEvent{entity: player, movement: IVec2::new(-1, -1)});
                        turns.progress_turn();
                    }
                    Some(KeyCode::Period) | Some(KeyCode::Numpad3) => {
                        ev_movement_event.send(PointMoveEvent{entity: player, movement: IVec2::new(1, -1)});
                        turns.progress_turn();
                    }

                    // Do Nothing
                    Some(KeyCode::K) | Some(KeyCode::Numpad5) => {
                        turns.progress_turn();
                    }

                    // Heal
                    Some(KeyCode::V) => {
                        ev_heal_event.send(HealActionEvent{healing_entity: player});
                        turns.progress_turn();
                    }

                    // Actions (TODO)
                    // Sploot!
                    // Heal
                    // (Melee doesn't need a keybind cause you just do it by walking into guys)

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

pub fn player_victory (
    enemy_query: Query<&TakesTurns, Without<Player>>,

    mut ev_actor_remove_event: EventReader<ActorRemovedEvent>,

    mut log: ResMut<Log>,
) {
    for ev in ev_actor_remove_event.iter() {
        if enemy_query.is_empty() {
            log.log_string_formatted(" You win! Press shift+r to start over.".to_string(), Color::YELLOW)
        }
    }
}

pub fn player_death (
    player_query: Query<&Player>,

    mut ev_actor_remove_event: EventReader<ActorRemovedEvent>,

    mut log: ResMut<Log>,
) {
    for ev in ev_actor_remove_event.iter() {
        if player_query.get(ev.removed_actor).is_ok() {
            log.log_string_formatted(" You have died! Press shift+r to try again.".to_string(), Color::YELLOW)
        }
    }
}
