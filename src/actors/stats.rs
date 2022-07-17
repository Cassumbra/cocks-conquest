use std::collections::BTreeMap;
use std::fmt::Display;

use bevy::prelude::*;

use crate::{data::Collides, rendering::Renderable, log::Log, turn::Turns};

use super::{TakesTurns, status_effects::{StatusEffectEvent, RemoveStatusEffectEvent, StatusEffects, StatusEffect, StatusEffectType, StatusEffectStacking}, ActorRemovedEvent};


// Systems
pub fn do_stat_change (
    mut ev_stat_change: EventReader<StatChangeEvent>,

    mut stats_query: Query<&mut Stats>,
) {
    for ev in ev_stat_change.iter() {
        if let Ok(mut stats) = stats_query.get_mut(ev.entity) {
            stats.get_mut(&ev.stat).unwrap().base += ev.amount;
            
            stats.get_mut(&ev.stat).unwrap().base = stats.get_base(&ev.stat).clamp(stats.get_min(&ev.stat), stats.get_max(&ev.stat));
        }
    }
}

pub fn update_effective_stats (
    mut ev_stat_change: EventReader<StatChangeEvent>,
    mut ev_status_effect: EventReader<StatusEffectEvent>,
    mut ev_removed_status_effect: EventReader<RemoveStatusEffectEvent>,

    mut stats_query: Query<(&mut Stats, Option<&StatusEffects>)>,
) {
    // TODO performance: THIS IS BAD. We are updating all stats on the actor instead of the changed stat. FIX THIS.
    let mut entities = Vec::<Entity>::new();
    
    // TODO performance: for these first two, we only need to push if the status actually has an effect on stats
    for ev in ev_removed_status_effect.iter() {
        entities.push(ev.entity);
    }
    for ev in ev_status_effect.iter() {
        entities.push(ev.entity);
    }
    for ev in ev_stat_change.iter() {
        entities.push(ev.entity);
    }

    // TODO performance: We are looping through all of our things to record things that need changes and then looping through them again. This is 2x more costly than it needs to be.
    for entity in entities {
        if let Ok((mut stats, opt_statuses)) = stats_query.get_mut(entity) {
            for (stattype, stat) in stats.iter_mut() {
                stat.effective = stat.base;
            }
            if let Some(statuses) = opt_statuses {
                for status in statuses.iter() {
                    if let Some(modification) = status.stat_modification {
                        
                        modification.compute(&modification.stat_type, &mut stats)
                    }
                }
            }

            // TODO: We should clamp the effective value here.
        }
    }
}

pub fn update_fatal (
    mut commands: Commands,

    mut ev_stat_change: EventReader<StatChangeEvent>,
    mut ev_actor_remove_event: EventWriter<ActorRemovedEvent>,
    mut ev_status_effect: EventWriter<StatusEffectEvent>,

    stats_query: Query<(Entity, &Stats, &FatalStats, Option<&Name>)>,
    mut renderable_query: Query<(&mut Renderable)>,

    mut log: ResMut<Log>,
    turns: Res<Turns>,
) {
    let mut fatalities = Vec::<(Entity, FatalEffect)>::new();

    'stat_changes: for ev in ev_stat_change.iter() {
        if let Ok((ent, stats, fatal_stats, opt_name)) = stats_query.get(ev.entity) {
            if let Some(stat) = stats.0.get(&ev.stat) {
                if let Some(fatal_stat_val) = fatal_stats.0.get(&ev.stat) {
                    if stat.effective == fatal_stat_val.0 {
                        let name = if opt_name.is_some() {opt_name.unwrap().to_string()} else {ev.entity.id().to_string()};

                        match &fatal_stats.0.get(&ev.stat).unwrap().1 {
                            FatalEffect::Disintegrate => {
                                // TODO: This is basically the same code in 3 different places. Can we do better than this?
                                for (fatal_ent, fatality) in &fatalities {
                                    if *fatal_ent == ent && *fatality == FatalEffect::Disintegrate {
                                        continue 'stat_changes;
                                    }
                                }

                                log.log_string_formatted(format!(" {} has died!", name), Color::ORANGE_RED);

                                ev_actor_remove_event.send(ActorRemovedEvent::new(ent, turns.count));

                                commands.entity(ent).despawn();

                                fatalities.push((ent, FatalEffect::Disintegrate));
                            }
                            FatalEffect::Corpse => {
                                for (fatal_ent, fatality) in &fatalities {
                                    if *fatal_ent == ent && *fatality == FatalEffect::Corpse {
                                        continue 'stat_changes;
                                    }
                                }

                                log.log_string_formatted(format!(" {} has died!", name), Color::ORANGE_RED);

                                commands.entity(ent)
                                    .remove::<TakesTurns>()
                                    .remove::<Collides>();
                                ev_actor_remove_event.send(ActorRemovedEvent::new(ent, turns.count));
        
                                if let Ok(mut renderable) = renderable_query.get_mut(ev.entity) {
                                    renderable.tile.bg_color = Color::ORANGE_RED;
                                }

                                fatalities.push((ent, FatalEffect::Corpse));
                            }
                            FatalEffect::Trance => {
                                for (fatal_ent, fatality) in &fatalities {
                                    if *fatal_ent == ent && *fatality == FatalEffect::Trance {
                                        continue 'stat_changes;
                                    }
                                }

                                log.log_string_formatted(format!(" {} has fallen under a trance!", name), Color::PINK);

                                ev_status_effect.send(StatusEffectEvent{
                                    effect: StatusEffect {
                                        status_type: StatusEffectType::Tranced,
                                        duration: None,
                                        stat_modification: None,
                                    },
                                    stacking: StatusEffectStacking::Refreshes,
                                    entity: ent,
                                });
                                if let Ok(mut renderable) = renderable_query.get_mut(ev.entity) {
                                    renderable.tile.bg_color = Color::PINK;
                                }

                                fatalities.push((ent, FatalEffect::Trance));
                            }
                        }
                    }
                }
            }
        }
    }
}

// Events
pub struct StatChangeEvent {
    pub stat: StatType,
    pub amount: i32,
    pub entity: Entity,
}
impl StatChangeEvent {
    pub fn new(stat: StatType, amount: i32, entity: Entity) -> Self {
        StatChangeEvent {stat, amount, entity}
    }
}

// Resources
/// If true, show hidden and private stats publicly.
#[derive(Deref, DerefMut)]
pub struct DebugShowStats (bool);
impl Default for DebugShowStats {
    fn default() -> Self {
        Self(false)
    }
}

// Data
#[derive(Clone, PartialEq)]
pub enum FatalEffect {
    Disintegrate,
    Corpse,
    Trance,
}

#[derive(Clone, PartialEq)]
pub enum StatVisibility {
    Public,
    Private,
    Hidden,
}

#[derive(Clone)]
pub struct Stat {
    pub base: i32,
    pub effective: i32,
    pub min: i32,
    pub max: i32,
    pub visibility: StatVisibility,
}
impl Stat {
    pub fn new(min: i32, max: i32, visibility: StatVisibility) -> Stat {
        Stat {base: max, effective: max, min, max, visibility}
    }

    pub fn with_value(value: i32, min: i32, max: i32, visibility: StatVisibility) -> Stat {
        Stat {base: value, effective: value, min, max, visibility}
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum StatType {
    Health,
    Resistance,

    CumPoints,

    Dexterity,
    Perception,
    Strength,

    StealthRange,
}
impl Display for StatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatType::Health => write!(f, "health"),
            StatType::Resistance => write!(f, "resistance"),

            StatType::CumPoints => write!(f, "cum points"),

            StatType::Dexterity => write!(f, "dexterity"),
            StatType::Perception => write!(f, "perception"),
            StatType::Strength => write!(f, "strength"),
            
            StatType::StealthRange => write!(f, "stealth range"),
        }
    }
}
impl StatType {
    pub fn color(&self) -> Color {
        match self {
            StatType::Health => Color::RED,
            StatType::Resistance => Color::BLUE,

            StatType::CumPoints => Color::WHITE,

            StatType::Dexterity => Color::GREEN,
            StatType::Perception => Color::PINK,
            StatType::Strength => Color::PINK,
            
            StatType::StealthRange => Color::GRAY,
        }
    }

    pub fn abbreviate(&self) -> String {
        match self {
            StatType::Health => String::from("hth"),
            StatType::Resistance => String::from("res"),

            StatType::CumPoints => String::from("cum"),

            StatType::Dexterity => String::from("dex"),
            StatType::Perception => String::from("per"),
            StatType::Strength => String::from("str"),
            
            StatType::StealthRange => String::from("sth"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StatModification {
    pub stat_type: StatType,
    pub operation: Operation,
}
impl StatModification {
    pub fn compute(&self, stat_type: &StatType, stats: &mut Stats) {
        stats.get_mut(stat_type).unwrap().effective = self.operation.compute(stats[stat_type].effective);
    }
    pub fn priority(&self) -> u8 {
        self.operation.priority()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Add(i32),
    Divide(i32),
    DivideRound(i32),
}
impl Operation {
    pub fn compute(&self, x: i32) -> i32 {
        match self {
            Operation::Add(n) => x + n,
            Operation::Divide(n) => x / n,
            Operation::DivideRound(n) => (x as f32 / *n as f32).round() as i32,
        }
    }
    pub fn priority(&self) -> u8 {
        match self {
            Operation::Add(_) => 2,
            Operation::Divide(_) => 1,
            Operation::DivideRound(_) => 1,
        }
    }
}

// Components
#[derive(Component, Clone, Deref, DerefMut)]
pub struct Stats(pub BTreeMap<StatType, Stat>);
impl Default for Stats {
    fn default() -> Stats {
        Stats(
            BTreeMap::from([
                (StatType::Health, Stat::new(0, 3, StatVisibility::Public)),
            ])
        )
    }
}
impl Stats {
    pub fn get_base (&self, stat: &StatType) -> i32 {
        // TODO: Check if we have the requested value. Otherwise, give 0 and print an error or something.
        if self.0.contains_key(stat) {
            self.0[&stat].base
        } else {
            eprintln!("ERROR: Stat not found! Returning zero.");
            0
        }
    }

    pub fn get_effective (&self, stat: &StatType) -> i32 {
        // TODO: Check if we have the requested value. Otherwise, give 0 and print an error or something.
        if self.0.contains_key(stat) {
            self.0[&stat].effective
        } else {
            eprintln!("ERROR: Stat not found! Returning zero.");
            0
        }
    }

    /*
    pub fn get_mut_value (&mut self, stat: &String) -> &mut i32 {
        &mut self.0.get_mut(stat).unwrap().value
    }
     */

    pub fn get_min (&self, stat: &StatType) -> i32 {
        self.0[&stat].min
    }

    pub fn get_max (&self, stat: &StatType) -> i32 {
        self.0[&stat].max
    }

    pub fn in_range (&self, stat: &StatType, value: i32) -> bool {
        value <= self.0[&stat].max && value >= self.0[&stat].min
    }
}

#[derive(Component, Clone)]
pub struct FatalStats(pub BTreeMap<StatType, (i32, FatalEffect)>);
impl Default for FatalStats {
    fn default() -> FatalStats {
        FatalStats(
            BTreeMap::from([
                (StatType::Health, (0, FatalEffect::Corpse)),
            ])
        )
    }
}

