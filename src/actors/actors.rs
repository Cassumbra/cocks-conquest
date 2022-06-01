use std::collections::BTreeMap;
use bevy::prelude::*;
use bevy_ascii_terminal::Tile;
use iyes_loopless::state::NextState;
use sark_grids::Grid;

use crate::{actions::{attack::{Attack, Dice}, melee::MeleeAttacker, ranged::{RangedAttacker, Projectile}}, map::{MapSize, Rooms}, data::{Position, Collides}, ai::{wander_behavior::Wanderer, targetting_behavior::Engages}, GameState, rendering::Renderable};


pub mod player;
use player::*;

pub mod vision;
use vision::*;

pub mod stats;
use stats::*;

use self::alignments::{Relations, Alignment};

pub mod status_effects;

pub mod alignments;


// Plugins
#[derive(Default)]
pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<stats::StatChangeEvent>()
        .add_event::<ActorRemovedEvent>();
    }
}

// Events
pub struct ActorRemovedEvent {
    pub removed_actor: Entity,
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
            .insert_bundle(SoldierBundle{
                vision: Vision( Map {
                    visible: Grid::default([map_size.width, map_size.height]),
                    ..Default::default()
                    }
                ),
                ..Default::default()
            })
            .insert(Position(room.center()))
            .insert(Name::new(format!("Soldier {}", i)))
            .insert(Wanderer::new(rooms.0.clone()));
    }
    

    commands.insert_resource(NextState(GameState::FinishSetup));
}

// Components
#[derive(Component, Default, Copy, Clone)]
pub struct TakesTurns;

#[derive(Component, Deref, DerefMut, Clone)]
pub struct Moves(pub Vec<IVec2>);
impl Default for Moves {
    fn default() -> Self {
        Moves(vec![
            // Cardinals first. We should only look at diagonals if they're faster.
            // We're going around the way we would go around a circle in radians too. Idk why, just feels right.
            IVec2::new(1, 0), IVec2::new(0, 1), IVec2::new(-1, 0), IVec2::new(0, -1),
            IVec2::new(1, 1), IVec2::new(-1, 1), IVec2::new(-1, -1), IVec2::new(1, -1),
        ])
    }
}

// TODO: Maybe we should make actors into an enum like how we have stats? Idk!

// Bundles
#[derive(Bundle, Clone)]
pub struct SoldierBundle {
    pub position: Position,
    pub renderable: Renderable,
    pub collides: Collides,
    pub takes_turns: TakesTurns,
    pub moves: Moves,
    pub vision: Vision,
    pub stats: Stats,
    pub fatal_stats: FatalStats,
    pub relations: Relations,
    pub engages: Engages,
    pub melee_attacker: MeleeAttacker,
    pub ranged_attacker: RangedAttacker,
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
            moves: Moves::default(),
            vision: Vision{..default()},
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
            relations: Relations::new(vec![Alignment::AntiCock], vec![Alignment::AntiCock], vec![Alignment::Cock]),
            engages: Engages {
                distance: 3.5,
                ..default()
            },
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
            ]},
            ranged_attacker: RangedAttacker{projectiles: vec![
                Projectile {
                    attack: Attack {
                        damage: Dice::new("1d2 * -1"),

                        ..default()
                    },

                    ..default()
                }
            ]},
        }
    }
}