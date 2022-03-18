
use std::collections::VecDeque;

// Unglob later
use bevy::{prelude::*, utils::HashMap};
use bevy_ascii_terminal::*;
use sark_grids::Grid;


#[derive(Component, Default, Copy, Clone)]
pub struct Collides;

// Shapes.

// A point.
#[derive(Component, Default, Copy, Clone)]
pub struct Position(pub IVec2);

#[derive(Component, Default, Copy, Clone, PartialEq)]
pub struct Rectangle {
    pub pos1: IVec2,
    pub pos2: IVec2,
}
impl Rectangle {
    pub fn new(pos: IVec2, width: i32, height: i32) -> Rectangle {
        Rectangle {pos1: pos, pos2: IVec2::new(pos.x + width, pos.y + height)}
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &Rectangle) -> bool {
        self.pos1.x <= other.pos2.x && self.pos2.x >= other.pos1.x && self.pos1.y <= other.pos2.y && self.pos2.y >= other.pos1.y
    }

    pub fn center(&self) -> IVec2 { 
        IVec2::new((self.pos1.x + self.pos2.x)/2, (self.pos1.y + self.pos2.y)/2)
    }
}

#[derive(Component)]
pub struct Turngon {
    pub vertices: Vec<IVec2>,
}

#[derive(Component)]
pub struct Polygon {
    pub vertices: Vec<IVec2>,
}

// Add component for turn-gon
// Polygon with only 90 degree turns
// Vec of tuples with two elements: turn right (bool) and distance i32/u32
// turn-gons will likely be more common than polygons. they should be useful, if not now, then later.