use bevy::utils::HashMap;
use bevy::prelude::{App, Plugin, Component};
use super::*;

pub mod ai;
use ai::*;

pub mod player;
use player::*;

// Systems
pub fn setup_actors (
    mut commands: Commands,
    rooms: Res<Rooms>,
) {
    let mut other_rooms = rooms.0.clone();
    let room_first = other_rooms.swap_remove(0);

    commands.spawn()
        .insert_bundle(PlayerBundle{..Default::default()})
        .insert(Position(room_first.center()))
        .insert(Name::new("Cass Cock"));

    for (i, room) in other_rooms.iter().enumerate() {
        commands.spawn()
            .insert_bundle(actors::SoldierBundle{..Default::default()})
            .insert(Position(room.center()))
            .insert(Name::new(format!("Soldier {}", i)))
            .insert(AI{..Default::default()});
    }
}

// Misc Data
#[derive(Clone)]
pub enum StatType{Health, Resistance, CumPoints}
impl Default for StatType {
    fn default() -> StatType {
        StatType::Health
    }
}

#[derive(Clone)]
pub struct Attack {
    pub interact_text: Vec<String>,
    pub damage: i32,
    pub damage_type: StatType,
    pub cost: Option<StatType>,
}
impl Default for Attack {
    fn default() -> Attack {
        Attack {
            interact_text: vec!["{attacker} hits {attacked} for {amount} damage!".to_string()],
            damage: 1,
            damage_type: StatType::Health,
            cost: None,
        }
    }
}

// Components
#[derive(Component, Default, Clone)]
pub struct Stats(HashMap<StatType, i32>);

#[derive(Component, Default, Copy, Clone)]
pub struct TakesTurns;

#[derive(Component, Clone, Default)]
pub struct MeleeAttacker {
    pub attacks: Vec<Attack>,
}


// Bundles
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