use std::collections::BTreeMap;

use bevy::prelude::*;

use crate::{components::Collides, rendering::Renderable, actions::interactions::ActorRemovedEvent};

use super::TakesTurns;


// Systems
pub fn do_stat_change (
    mut ev_stat_change: EventReader<StatChangeEvent>,

    mut stats_query: Query<&mut Stats>,
) {
    for ev in ev_stat_change.iter() {
        if let Ok(mut stats) = stats_query.get_mut(ev.entity) {
            stats.0.get_mut(&ev.stat).unwrap().value += ev.amount;
        }
    }
}

pub fn update_fatal (
    mut commands: Commands,

    mut ev_stat_change: EventReader<StatChangeEvent>,
    mut ev_actor_remove_event: EventWriter<ActorRemovedEvent>,

    stats_query: Query<(Entity, &Stats, &FatalStats)>,
    mut renderable_query: Query<(&mut Renderable)>,
) {
    // TODO: logging

    for ev in ev_stat_change.iter() {
        if let Ok((ent, stats, fatal_stats)) = stats_query.get(ev.entity) {
            if let Some(stat) = stats.0.get(&ev.stat) {
                if let Some(fatal_stat_val) = fatal_stats.0.get(&ev.stat) {
                    if stat.value == fatal_stat_val.0 {
                        match &fatal_stats.0.get(&ev.stat).unwrap().1 {
                            FatalEffect::Disintegrate => {
                                commands.entity(ent).despawn();
                            }
                            FatalEffect::Corpse => {
                                commands.entity(ent)
                                    .remove::<TakesTurns>()
                                    .remove::<Collides>();
                                ev_actor_remove_event.send(ActorRemovedEvent);
        
                                if let Ok(mut renderable) = renderable_query.get_mut(ev.entity) {
                                    renderable.tile.bg_color = Color::ORANGE_RED;
                                }
                            }
                            FatalEffect::Trance => {
                                // We should add a visual effect too.
                                commands.entity(ent).insert(Tranced);
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
    pub stat: String,
    pub amount: i32,
    pub entity: Entity,
}

// Data
#[derive(Clone)]
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

// Components
#[derive(Component, Clone)]
pub struct Stats(pub BTreeMap<String, Stat>);
impl Default for Stats {
    fn default() -> Stats {
        Stats(
            BTreeMap::from([
                ("health".to_string(), Stat{value: 3, min: 0, max: 3}),
            ])
        )
    }
}
impl Stats {
    pub fn get_value (&self, stat: &String) -> i32 {
        self.0[stat].value
    }

    pub fn get_min (&self, stat: &String) -> i32 {
        self.0[stat].min
    }

    pub fn get_max (&self, stat: &String) -> i32 {
        self.0[stat].max
    }

    pub fn in_range (&self, stat: &String, value: i32) -> bool {
        value <= self.0[stat].max && value >= self.0[stat].min
    }
}

#[derive(Component, Clone)]
pub struct FatalStats(pub BTreeMap<String, (i32, FatalEffect)>);
impl Default for FatalStats {
    fn default() -> FatalStats {
        FatalStats(
            BTreeMap::from([
                ("health".to_string(), (0, FatalEffect::Corpse)),
            ])
        )
    }
}

#[derive(Component, Clone)]
pub struct Tranced;
