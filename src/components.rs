// Unglob later
use bevy::prelude::*;
use bevy_ascii_terminal::*;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/*
pub enum Bg {
    Transparent,
    Opaque (Color),
}*/

#[derive(Component)]
pub struct Renderable {
    pub tile: Tile,
    pub order: u8,
}

#[derive(Component)]
pub struct Name {
    pub name: String,
}