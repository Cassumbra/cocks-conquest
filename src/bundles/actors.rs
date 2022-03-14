// Unglob later
use bevy::prelude::*;
use super::super::*;

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

#[derive(Bundle, Copy, Clone)]
pub struct SoldierBundle {
    pub position: Position,
    pub renderable: Renderable,
    pub collides: Collides,
    pub takes_turns: TakesTurns,
}
impl Default for SoldierBundle {
    fn default() -> SoldierBundle {
        SoldierBundle {
            position: Position (IVec2::new(0, 0)),
            renderable: Renderable {
                tile: Tile {
                    glyph: 'H',
                    fg_color: Color::GRAY,
                    bg_color: Color::NONE,
                },
                order: 128
            },
            collides: Collides,
            takes_turns: TakesTurns,
        }
    }
}