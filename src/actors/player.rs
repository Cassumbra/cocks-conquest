use bevy::input::{keyboard::KeyboardInput, ElementState};
use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::utils::HashMap;
use bevy_ascii_terminal::Tile;
use crate::actions::movement::PointMoveEvent;
use crate::components::Collides;
use crate::rendering::Renderable;
use crate::rendering::window::WindowChangeEvent;

use super::actors::TakesTurns;
use super::{Position, Turns, Vision, MindMap, Stats, StatType};


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
}
impl Default for PlayerBundle {
    fn default() -> PlayerBundle {

        let mut stat_data: HashMap<StatType, i32> = HashMap::default();
        stat_data.insert(StatType::Health, 3);
        stat_data.insert(StatType::CumPoints, 20);

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
            stats: Stats(stat_data),
        }
    }
}

// Systems
/// Player input.
/// 
pub fn player_input(
    query: Query<Entity, (With<Position>, With<Player>, With<TakesTurns>)>,

    mut ev_key: EventReader<KeyboardInput>,
    mut ev_movement_event: EventWriter<PointMoveEvent>,
    mut ev_exit: EventWriter<AppExit>,
    mut ev_window_change: EventWriter<WindowChangeEvent>,

    mut turns: ResMut<Turns>,
) {
    for ent in query.iter() {
        if turns.is_turn(&ent) {
            for ev in ev_key.iter() {
                if ev.state == ElementState::Pressed {
                    match ev.key_code {
                        // Cardinal Movement
                        Some(KeyCode::I) | Some(KeyCode::Numpad8) => {
                            ev_movement_event.send(PointMoveEvent{entity: ent, movement: IVec2::new(0, 1)});
                            turns.progress_turn();
                        }
                        Some(KeyCode::Comma) | Some(KeyCode::Numpad2) => {
                            ev_movement_event.send(PointMoveEvent{entity: ent, movement: IVec2::new(0, -1)});
                            turns.progress_turn();
                        }
                        Some(KeyCode::J) | Some(KeyCode::Numpad4) => {
                            ev_movement_event.send(PointMoveEvent{entity: ent, movement: IVec2::new(-1, 0)});
                            turns.progress_turn();
                        }
                        Some(KeyCode::L) | Some(KeyCode::Numpad6) => {
                            ev_movement_event.send(PointMoveEvent{entity: ent, movement: IVec2::new(1, 0)});
                            turns.progress_turn();
                        }
            
                        // Diagonal Movement
                        Some(KeyCode::U) | Some(KeyCode::Numpad7) => {
                            ev_movement_event.send(PointMoveEvent{entity: ent, movement: IVec2::new(-1, 1)});
                            turns.progress_turn();
                        }
                        Some(KeyCode::O) | Some(KeyCode::Numpad9) => {
                            ev_movement_event.send(PointMoveEvent{entity: ent, movement: IVec2::new(1, 1)});
                            turns.progress_turn();
                        }
                        Some(KeyCode::M) | Some(KeyCode::Numpad1) => {
                            ev_movement_event.send(PointMoveEvent{entity: ent, movement: IVec2::new(-1, -1)});
                            turns.progress_turn();
                        }
                        Some(KeyCode::Period) | Some(KeyCode::Numpad3) => {
                            ev_movement_event.send(PointMoveEvent{entity: ent, movement: IVec2::new(1, -1)});
                            turns.progress_turn();
                        }

                        // Do Nothing
                        Some(KeyCode::K) | Some(KeyCode::Numpad5) => {
                            turns.progress_turn();
                        }

                        // Actions (TODO)
                        // Sploot!
                        // Heal
                        // (Melee doesn't need a keybind cause you just do it by walking into guys)

                        // Other stuff
                        Some(KeyCode::Escape) => {
                            ev_exit.send(AppExit);
                        }
                        Some(KeyCode::NumpadAdd) | Some(KeyCode::Equals) => {
                            ev_window_change.send(WindowChangeEvent(1));
                        }
                        Some(KeyCode::NumpadSubtract) | Some(KeyCode::Minus) => {
                            ev_window_change.send(WindowChangeEvent(-1));
                        }

                        _ => {}
                    }
                }
            }
        }
    }
}