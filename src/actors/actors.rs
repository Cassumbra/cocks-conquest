use std::collections::BTreeMap;
use bevy::prelude::*;
use sark_grids::Grid;

use crate::actions::interactions::{MeleeAttacker, Attack};

use super::*;

pub mod ai;
use ai::*;

pub mod player;
use player::*;

pub mod vision;
use vision::*;

pub mod stats;
use stats::*;


// Plugins
#[derive(Default)]
pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<stats::StatChangeEvent>();
    }
}

// Systems
pub fn setup_actors (
    mut commands: Commands,
    
    rooms: Res<Rooms>,
    map_size: Res<MapSize>,
) {
    let mut other_rooms = rooms.0.clone();
    let room_first = other_rooms.swap_remove(0);

    commands.spawn()
        .insert_bundle(PlayerBundle{
            vision: Vision( Map {
                visible: Grid::default([map_size.width, map_size.height]),
                ..Default::default()
                }
            ),
            mind_map: MindMap{
                seen: Grid::default([map_size.width, map_size.height]),
            },
            ..Default::default()
        })
        .insert(Position(room_first.center()))
        .insert(Name::new("Cass Cock"));

    

    for (i, room) in other_rooms.iter().enumerate() {
        commands.spawn()
            .insert_bundle(actors::SoldierBundle{
                vision: Vision( Map {
                    visible: Grid::default([map_size.width, map_size.height]),
                    ..Default::default()
                    }
                ),
                ..Default::default()
            })
            .insert(Position(room.center()))
            .insert(Name::new(format!("Soldier {}", i)))
            .insert(AI{..Default::default()});
    }
}

#[derive(Component, Default, Copy, Clone)]
pub struct TakesTurns;


// Bundles
#[derive(Bundle, Clone)]
pub struct SoldierBundle {
    pub position: Position,
    pub renderable: Renderable,
    pub collides: Collides,
    pub takes_turns: TakesTurns,
    pub vision: Vision,
    pub stats: Stats,
    pub fatal_stats: FatalStats,
    pub melee_attacker: MeleeAttacker,
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
            vision: Vision{..Default::default()},
            stats: Stats(
                BTreeMap::from([
                    ("health".to_string(), Stat{value: 3, min: 0, max: 3}),
                    ("resistance".to_string(), Stat{value: 3, min: 0, max: 3}),
                ])
            ),
            fatal_stats: FatalStats(
                BTreeMap::from([
                    ("health".to_string(), (0, FatalEffect::Corpse)),
                    ("resistance".to_string(), (0, FatalEffect::Trance))
                ])
            ),
            melee_attacker: MeleeAttacker{attacks: vec![
                Attack{
                    interact_text: vec!["{attacker} stabs {attacked} for {amount} damage!".to_string(),
                                        "{attacker} slashes {attacked} for {amount} damage!".to_string(),],
                    damage: -1,
                    damage_type: "health".to_string(),
                    cost: 0,
                    cost_type: "health".to_string(),
                }
            ]}
        }
    }
}