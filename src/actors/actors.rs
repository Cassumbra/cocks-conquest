use std::collections::BTreeMap;
use bevy::{prelude::*, ecs::event::Events};
use bevy_ascii_terminal::Tile;
use iyes_loopless::state::NextState;
use sark_grids::Grid;

use crate::{actions::{attack::{Attack, Dice}, melee::MeleeAttacker, ranged::{RangedAttacker, Projectile}, vore::DoesVore, healing::CanHeal}, map::{MapSize, Rooms}, data::{Position, Collides}, ai::{wander_behavior::Wanderer, targetting_behavior::Engages}, GameState, rendering::Renderable, turn::TurnEvent, player::Player};


pub mod vision;
use vision::*;

pub mod stats;
use stats::*;

pub mod status_effects;

pub mod temp_stats;

pub mod alignments;
use self::{alignments::{Relations, Alignment}, status_effects::{StatusEffectEvent, RemoveStatusEffectEvent, StatusEffects, StatusEffectApplication, StatusEffect, StatusEffectType, StatusEffectStacking, TileModification}};

// Plugin
#[derive(Default)]
pub struct ActorPlugin;
impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<stats::StatChangeEvent>()
        .init_resource::<Events<ActorRemovedEvent>>()
        .init_resource::<Events<StatusEffectEvent>>()
        .init_resource::<Events<RemoveStatusEffectEvent>>()
        .init_resource::<DebugShowStats>();
    }
}

// Events
pub struct ActorRemovedEvent {
    pub removed_actor: Entity,
    pub turn: u32,
    has_run: bool,
}
impl ActorRemovedEvent {
    pub fn new(removed_actor: Entity, turn: u32) -> Self{
        ActorRemovedEvent { removed_actor, turn, has_run: false }
    }
}
impl TurnEvent for ActorRemovedEvent {
    fn get_turn(&self) -> u32 {
        self.turn
    }
    fn update(&mut self) {
        self.has_run = true;
    }
    fn frame_valid(&self) -> bool {
        !self.has_run
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
pub struct PlayerBundle {
    pub position: Position,
    pub renderable: Renderable,
    pub collides: Collides,
    pub player: Player,
    pub takes_turns: TakesTurns,
    pub vision: Vision,
    pub mind_map: MindMap,
    pub status_effects: StatusEffects,
    pub stats: Stats,
    pub fatal_stats: FatalStats,
    pub relations: Relations,
    pub melee_attacker: MeleeAttacker,
    pub ranged_attacker: RangedAttacker,
    pub does_vore: DoesVore,
    pub can_heal: CanHeal,
}
impl Default for PlayerBundle {
    fn default() -> PlayerBundle {
        PlayerBundle {
            position: Position (IVec2::new(0, 0)),
            renderable: Renderable::new(
                Tile {
                    glyph: '@',
                    fg_color: Color::RED,
                    bg_color: Color::NONE,
                },
                128
            ),
            collides: Collides,
            player: Player,
            takes_turns: TakesTurns,
            vision: Vision{..Default::default()},
            mind_map: MindMap{..Default::default()},
            status_effects: StatusEffects::default(),
            stats: Stats(
                BTreeMap::from([
                    (StatType::Health, Stat::new(0, 7, StatVisibility::Public)),
                    (StatType::CumPoints, Stat::with_value(15, 0, 60, StatVisibility::Private)),
                    (StatType::StealthRange, Stat::with_value(7, 1, 128, StatVisibility::Private)),
                ])
            ),
            fatal_stats: FatalStats{..Default::default()},
            relations: Relations::new(vec![Alignment::Cock], vec![Alignment::Cock], vec![Alignment::AntiCock]),
            melee_attacker: MeleeAttacker{attacks: vec![
                Attack{
                    interact_text: vec!["{attacker} breathes their stink into {attacked}'s head, lowering their resistance by {amount}!".to_string(),
                                        "{attacker} gives {attacked} a big smelly kiss with their cockmaw, lowering their resistance by {amount}!".to_string(),],
                    damage: Dice::new("1d4 * -1"),
                    damage_type: StatType::Resistance,
                    cost: Dice::new("0"),
                    cost_type: StatType::Health,

                    // Temporary for now
                    ..default()
                }
            ]},
            ranged_attacker: RangedAttacker{projectiles: vec![
                Projectile {
                    count: 1,
                    attack: Attack {
                        damage: Dice::new("1d4 * -1"),
                        damage_type: StatType::Resistance,
                        cost: Dice::new("0"),
                        cost_type: StatType::CumPoints,

                        status_effect: Some(
                            StatusEffectApplication {
                                effect: StatusEffect {
                                    status_type: StatusEffectType::Cumblobbed,
                                    tile_modification: Some(TileModification {glyph: None, fg_color: None, bg_color: Some(Color::ANTIQUE_WHITE)}),
                                    duration: Some(3),
                                    stat_modification: None,
                                },
                                stacking: StatusEffectStacking::Refreshes,
                            },
                        ),

                        ..default()
                    },

                    cost: Dice::new("-5"),
                    cost_type: StatType::CumPoints,
                    ..default()
                }
            ]},
            does_vore: DoesVore,
            can_heal: CanHeal,
        }
    }
}

#[derive(Bundle, Clone)]
pub struct SoldierBundle {
    pub position: Position,
    pub renderable: Renderable,
    pub collides: Collides,
    pub takes_turns: TakesTurns,
    pub moves: Moves,
    pub vision: Vision,
    pub status_effects: StatusEffects,
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
            renderable: Renderable::new(
                Tile {
                    glyph: 'H',
                    fg_color: Color::GRAY,
                    bg_color: Color::NONE,
                },
                128
            ),
            collides: Collides,
            takes_turns: TakesTurns,
            moves: Moves::default(),
            vision: Vision{..default()},
            status_effects: StatusEffects::default(),
            stats: Stats(
                BTreeMap::from([
                    (StatType::Health, Stat::new(0, 7, StatVisibility::Public)),
                    (StatType::Resistance, Stat::new(0, 7, StatVisibility::Public)),
                    (StatType::Perception, Stat::new(0, 5, StatVisibility::Hidden)),
                    (StatType::Dexterity, Stat::new(0, 5, StatVisibility::Hidden)),
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
                    ..default()
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