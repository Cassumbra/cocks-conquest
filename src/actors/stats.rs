use std::collections::BTreeMap;

use bevy::prelude::*;

use crate::{components::Collides, rendering::Renderable, actions::interactions::ActorRemovedEvent, log::Log};

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

    stats_query: Query<(Entity, &Stats, &FatalStats, Option<&Name>)>,
    mut renderable_query: Query<(&mut Renderable)>,

    mut log: ResMut<Log>,
) {
    // TODO: logging

    for ev in ev_stat_change.iter() {
        if let Ok((ent, stats, fatal_stats, opt_name)) = stats_query.get(ev.entity) {
            if let Some(stat) = stats.0.get(&ev.stat) {
                if let Some(fatal_stat_val) = fatal_stats.0.get(&ev.stat) {
                    if stat.value == fatal_stat_val.0 {
                        let name = if opt_name.is_some() {opt_name.unwrap().to_string()} else {ev.entity.id().to_string()};

                        match &fatal_stats.0.get(&ev.stat).unwrap().1 {
                            FatalEffect::Disintegrate => {
                                log.log_string_formatted(format!(" {} has died!", name), Color::ORANGE_RED);

                                commands.entity(ent).despawn();
                            }
                            FatalEffect::Corpse => {
                                log.log_string_formatted(format!(" {} has died!", name), Color::ORANGE_RED);

                                commands.entity(ent)
                                    .remove::<TakesTurns>()
                                    .remove::<Collides>();
                                ev_actor_remove_event.send(ActorRemovedEvent{removed_actor: ent});
        
                                if let Ok(mut renderable) = renderable_query.get_mut(ev.entity) {
                                    renderable.tile.bg_color = Color::ORANGE_RED;
                                }
                            }
                            FatalEffect::Trance => {
                                log.log_string_formatted(format!(" {} has fallen under a trance!", name), Color::PINK);

                                commands.entity(ent).insert(Tranced);
                                if let Ok(mut renderable) = renderable_query.get_mut(ev.entity) {
                                    renderable.tile.bg_color = Color::PINK;
                                }
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
