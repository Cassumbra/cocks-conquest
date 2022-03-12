// Unglob later
use bevy::{prelude::*};
use sark_grids::Grid;
use super::*;

#[derive(Default)]
pub struct RenderOrder(pub Vec<Entity>);

#[derive(Default)]
pub struct Rooms(pub Vec<Rectangle>);

//#[derive(Default)]
//pub struct MapObjects(pub Grid<Option<Entity>>);

#[derive(Default, Clone)]
pub struct Collidables(pub Grid<Option<Entity>>);

#[derive(Default)]
pub struct Turns {
    pub order: Vec<Entity>,
    pub current: usize,
    pub progress: bool,
}
impl Turns {
    pub fn is_turn(&self, entity: &Entity) -> bool {
        //println!("{:?}", self.order);
        self.order.len() > 0 && self.order[self.current] == *entity && !self.progress
    }
    pub fn progress_turn(&mut self) {
        self.progress = true;
    }

}


// We need to make a more sophisticated way for doing this the next time we do this.
// This should work for now, though.

// TODO: When remaking this, create display entities with width and height. Have a ScreenSize resource.
// Display entities are parents of entities with a renderable component.
// ScreenSize determines the max size of display entities.
// Display entities can be made with a rectangle component and a display tag component.
// Future me: Take these plans at your own discretion. Also, don't bother working on any of this shit if you're not even future me.
pub struct MapSize {
    pub width: u32,
    pub height: u32,
}
impl Default for MapSize {
    fn default() -> MapSize {
        MapSize {
            width: 80,
            height: 40,
        }
    }
}

pub struct BottomSize {
    pub height: u32,
}
impl Default for BottomSize {
    fn default() -> BottomSize {
        BottomSize {
            height: 10,
        }
    }
}

pub struct SpriteMagnification (pub u32);
impl Default for SpriteMagnification {
    fn default() -> SpriteMagnification {
        SpriteMagnification(1)
    }
}


