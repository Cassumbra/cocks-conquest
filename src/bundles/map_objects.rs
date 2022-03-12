// Unglob later
use bevy::prelude::*;
use super::super::*;

#[derive(Bundle, Copy, Clone)]
pub struct WallBundle {
    pub position: Position,
    pub renderable: Renderable,
    pub collides: Collides,
}
impl Default for WallBundle {
    fn default() -> WallBundle {
        WallBundle {
            position: Position (IVec2::new(0, 0)),
            renderable: Renderable {
                tile: Tile {
                    glyph: '#',
                    fg_color: Color::WHITE,
                    bg_color: Color::BLACK,
                },
                order: 32
            },
            collides: Collides,
        }
    }
}

#[derive(Bundle, Copy, Clone)]
pub struct FloorBundle {
    pub position: Position,
    pub renderable: Renderable,
}
impl Default for FloorBundle {
    fn default() -> FloorBundle {
        FloorBundle {
            position: Position (IVec2::new(0, 0)),
            renderable: Renderable {
                tile: Tile {
                    glyph: '.',
                    fg_color: Color::DARK_GRAY,
                    bg_color: Color::BLACK,
                },
                order: 48
            },
        }
    }
}