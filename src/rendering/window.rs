// Unglob later
use bevy::prelude::*;
use super::super::*;

//Plugin
#[derive(Default)]
pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<WindowChangeEvent>()
        .init_resource::<SpriteMagnification>();
    }
}


//Events
pub struct WindowChangeEvent(pub i32);

//Resources
pub struct SpriteMagnification (pub u32);
impl Default for SpriteMagnification {
    fn default() -> SpriteMagnification {
        SpriteMagnification(1)
    }
}

//Systems
pub fn change_size(
    mut ev_window_change: EventReader<WindowChangeEvent>,
    map_size: Res<MapSize>,
    bottom_size: Res<BottomSize>,
    mut sprite_magnification: ResMut<SpriteMagnification>,
    mut windows:  ResMut<Windows>,
) {
    if let Some(window_change) = ev_window_change.iter().next() {
        // Unreadable garbage below lol
        let screen_size_width = map_size.width;
        let screen_size_height = map_size.height + bottom_size.height;
        let change = window_change.0;
    
        let mag_to_be = sprite_magnification.0 as i32 + change;
        sprite_magnification.0 = mag_to_be.max(1).min(4) as u32;
    
        let window = windows.get_primary_mut().unwrap();
        window.set_resolution((8 * sprite_magnification.0 * screen_size_width) as f32, (8 * sprite_magnification.0 * screen_size_height) as f32);
        // WARNING: 8 IS A MAGIC NUMBER FOR OUR SPRITE SIZE. CRINGE ALERT!!!
    }
}