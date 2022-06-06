use std::collections::BTreeMap;
use std::fmt::Display;

use bevy::prelude::*;

use crate::{data::Collides, rendering::Renderable, log::Log, turn::Turns};

use super::{TakesTurns, status_effects::Tranced, ActorRemovedEvent};


// Systems
pub fn do_stat_change (
    mut ev_stat_change: EventReader<StatChangeEvent>,

    mut stats_query: Query<&mut Stats>,
) {
    for ev in ev_stat_change.iter() {
        if let Ok(mut stats) = stats_query.get_mut(ev.entity) {
            stats.0.get_mut(&ev.stat).unwrap().value += ev.amount;
            
            stats.0.get_mut(&ev.stat).unwrap().value = stats.get_value(&ev.stat).clamp(stats.get_min(&ev.stat), stats.get_max(&ev.stat));
        }
    }
}

pub fn update_fatal (
    mut commands: Commands,

    mut ev_stat_change: EventReader<StatChangeEvent>,
    mut ev_actor_remove_event: EventWriter<ActorRemovedEvent>,

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
                    if stat.value == fatal_stat_val.0 {
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

                                commands.entity(ent).insert(Tranced);
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

// Data
#[derive(Clone, PartialEq)]
pub enum FatalEffect{
    Disintegrate,
    Corpse,
    Trance,
}

#[derive(Clone)]
pub struct Stat {
    pub value: i32,
    pub min: i32,
    pub max: i32,
}
impl Stat {
    pub fn new(min: i32, max: i32) -> Stat {
        Stat {value: max, min, max}
    }

    pub fn with_value(value: i32, min: i32, max: i32) -> Stat {
        Stat {value, min, max}
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StatType {
    Health,
    Resistance,

    CumPoints,

    Dexterity,
    Perception,
    Strength,
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
            
        }
    }
}

// Components
#[derive(Component, Clone)]
pub struct Stats(pub BTreeMap<StatType, Stat>);
impl Default for Stats {
    fn default() -> Stats {
        Stats(
            BTreeMap::from([
                (StatType::Health, Stat{value: 3, min: 0, max: 3}),
            ])
        )
    }
}
impl Stats {
    pub fn get_value (&self, stat: &StatType) -> i32 {
        // TODO: Check if we have the requested value. Otherwise, give 0 and print an error or something.
        if self.0.contains_key(stat) {
            self.0[&stat].value
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

