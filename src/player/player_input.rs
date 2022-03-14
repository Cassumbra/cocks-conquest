use bevy::input::{keyboard::KeyboardInput, ElementState};
// Unglob later
use bevy::prelude::*;
use bevy::app::AppExit;
use super::super::*;

/// Player input.
/// 
pub fn player_input(
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
    query: Query<Entity, (With<Position>, With<Player>, With<IsTurn>)>,
    mut key_evr: EventReader<KeyboardInput>,
    mut ev_movement_event: EventWriter<PointMoveEvent>,
    map_size: Res<MapSize>,
    bottom_size: Res<BottomSize>,
    mut sprite_magnification: ResMut<SpriteMagnification>,
    mut windows: ResMut<Windows>,
    mut turns: ResMut<Turns>,
) {
    for ent in query.iter() {
        for ev in key_evr.iter() {
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
                        exit.send(AppExit);
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