use bevy::input::{keyboard::KeyboardInput, ElementState};
use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_ascii_terminal::Tile;
use crate::actions::movement::PointMoveEvent;
use crate::components::Collides;
use crate::rendering::Renderable;

use super::actors::TakesTurns;
use super::{rendering, BottomSize, Position, SpriteMagnification, MapSize, Turns};


// Components
#[derive(Component, Default, Copy, Clone)]
pub struct Player;

// Bundles
#[derive(Bundle, Copy, Clone)]
pub struct PlayerBundle {
    pub position: Position,
    pub renderable: Renderable,
    pub collides: Collides,
    pub player: Player,
    pub takes_turns: TakesTurns,
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
    //mut ev_window_change: EventWriter<WindowChangeEvent>,

    map_size: Res<MapSize>,
    bottom_size: Res<BottomSize>,
    mut sprite_magnification: ResMut<SpriteMagnification>,
    mut windows: ResMut<Windows>,
    mut turns: ResMut<Turns>,
) {
    for ent in query.iter() {
        if turns.is_turn(&ent) {
            for ev in ev_key.iter() {
                if ev.state == ElementState::Pressed {
                    match ev.key_code {
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

                        // Other stuff
                        Some(KeyCode::Escape) => {
                            ev_exit.send(AppExit);
                        }
                        Some(KeyCode::NumpadAdd) | Some(KeyCode::Equals) => {
                            rendering::window::change_size(1, &map_size, &bottom_size, &mut sprite_magnification, &mut windows)
                        }
                        Some(KeyCode::NumpadSubtract) | Some(KeyCode::Minus) => {
                            rendering::window::change_size(-1, &map_size, &bottom_size, &mut sprite_magnification, &mut windows)
                        }

                        _ => {}
                    }
                }
            }
        }
    }
}