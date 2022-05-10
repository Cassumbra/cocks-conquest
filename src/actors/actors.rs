use std::collections::BTreeMap;
use bevy::prelude::*;
use sark_grids::Grid;

use crate::actions::interactions::{MeleeAttacker, Attack, Dice};

use super::*;

pub mod ai;
use ai::*;

pub mod player;
use player::*;

pub mod vision;
use vision::*;

pub mod stats;
use stats::*;

pub mod status_effects;


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
    

    commands.insert_resource(NextState(GameState::FinishSetup));
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
                    (StatType::Health, Stat::new(0, 7)),
                    (StatType::Resistance, Stat::new(0, 7)),
                ])
            ),
            fatal_stats: FatalStats(
                BTreeMap::from([
                    (StatType::Health, (0, FatalEffect::Corpse)),
                    (StatType::Resistance, (0, FatalEffect::Trance))
                ])
            ),
            melee_attacker: MeleeAttacker{attacks: vec![
                Attack {
                    interact_text: vec!["{attacker} stabs {attacked} for {amount} damage!".to_string(),
                                        "{attacker} slashes {attacked} for {amount} damage!".to_string(),],
                    damage: Dice::new("1d4 * -1"),
                    damage_type: StatType::Health,
                    cost: Dice::new("0"),
                    cost_type: StatType::Health,
                    save_text: vec![String::from("{attacked} maneuvers out of {attacker's} stab!")],
                    save: 16,
                    save_type: StatType::Dexterity,
                }
            ]}
        }
    }
}