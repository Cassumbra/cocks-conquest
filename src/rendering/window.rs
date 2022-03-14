// Unglob later
use bevy::prelude::*;
use super::super::*;

pub fn change_size(
    change: i32,
    map_size: &Res<MapSize>,
    bottom_size: &Res<BottomSize>,
    sprite_magnification: &mut ResMut<SpriteMagnification>,
    windows: &mut ResMut<Windows>,
) {
    let screen_size_width = map_size.width;
    let screen_size_height = map_size.height + bottom_size.height;

    let mag_to_be = sprite_magnification.0 as i32 + change;
    sprite_magnification.0 = mag_to_be.max(1).min(4) as u32;

    let window = windows.get_primary_mut().unwrap();
    window.set_resolution((8 * sprite_magnification.0 * screen_size_width) as f32, (8 * sprite_magnification.0 * screen_size_height) as f32);
    // WARNING: 8 IS A MAGIC NUMBER FOR OUR SPRITE SIZE. CRINGE ALERT!!!
}