use bevy::prelude::*;
use std::fmt::Display;

use crate::turn::Turns;

use super::{TakesTurns, stats::{StatModification, StatType}};





// Systems
// I don't know if I want to move this to be with the rest of the behaviors or not.
// It may not only affect AI.
pub fn tranced_behavior (
    mut turns: ResMut<Turns>,

    tranced_query: Query<&Tranced, With<TakesTurns>>,
) {
    // TODO: Maybe we should turn this into a system condition?
    if turns.progress == true {
        println!("no trancing for me!");
        return;
    }

    let entity = turns.order[turns.current];
    if let Ok(_tranced) = tranced_query.get(entity) {
        println!("wuhh im tranced woah");
        turns.progress_turn();
    }
}

pub fn update_status_effects (
    turns: Res<Turns>,

    mut status_query: Query<(Entity, &mut StatusEffects)>,

    mut ev_removed_status_effect: EventWriter<RemoveStatusEffectEvent>,
) {
    if turns.progress {
        for (entity, mut statuses) in status_query.iter_mut() {
            let length = statuses.len();
            for status in statuses.iter_mut() {
                if let Some(duration) = status.duration.as_mut() {
                    *duration -= 1;
                }
            }

            // TODO: REAL???
            statuses.retain(|a| a.duration < Some(1));

            if statuses.len() != length {
                ev_removed_status_effect.send(RemoveStatusEffectEvent{entity});
            }
        }
    }
}

pub fn apply_status_effects (
    mut ev_status_effect: EventReader<StatusEffectEvent>,

    mut status_query: Query<&mut StatusEffects>,
) {
    for ev in ev_status_effect.iter() {
        let mut append = false;

        if let Ok(mut statuses) = status_query.get_mut(ev.entity) {
            for status in statuses.iter_mut() {
                if status.status_type == ev.effect.status_type {
                    match ev.stacking {
                        StatusEffectStacking::Stacks => {
                            append = true;
                            break
                        }
                        StatusEffectStacking::Adds => {
                            if let Some(duration) = status.duration.as_mut() {
                                if let Some(ev_duration) = ev.effect.duration {
                                    *duration += ev_duration;
                                }
                                else {
                                    status.duration = None;
                                }
                            }
                            
                        }
                        StatusEffectStacking::Refreshes => {
                            status.duration = ev.effect.duration;
                        },
                    }
                }
                else {
                    append = true;
                    break
                }
            }
        }

        if append {
            if let Ok(mut statuses) = status_query.get_mut(ev.entity) {
                statuses.push(ev.effect);

                statuses.sort_by(|a, b| a.priority().cmp(&b.priority()))
            }
        }
    }
}

// Components
#[derive(Component, Clone)]
pub struct Tranced;

#[derive(Component, Clone, Deref, DerefMut)]
pub struct StatusEffects (Vec<StatusEffect>);

// Data
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StatusEffectType {
    Tranced,
    Sneaking,
    Cumblobbed,
    Restrained,
}
impl Display for StatusEffectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusEffectType::Tranced => write!(f, "tranced"),
            StatusEffectType::Sneaking => write!(f, "sneaking"),
            StatusEffectType::Cumblobbed => write!(f, "cumblobbed"),
            StatusEffectType::Restrained => write!(f, "restrained"),
        }
    }
}
impl StatusEffectType {
    pub fn color(&self) -> Color {
        match self {
            StatusEffectType::Tranced => Color::PINK,
            StatusEffectType::Sneaking => Color::GRAY,
            StatusEffectType::Cumblobbed => Color::WHITE,
            StatusEffectType::Restrained => Color::YELLOW,
        }
    }
    pub fn abbreviate(&self) -> String {
        match self {
            StatusEffectType::Tranced => String::from("trn"),
            StatusEffectType::Sneaking => String::from("snk"),
            StatusEffectType::Cumblobbed => String::from("cum"),
            StatusEffectType::Restrained => String::from("rst"),
        }
    }
}

// Determines how a status effect interacts with other statuses of the same kind.
#[derive(Clone, Copy)]
pub enum StatusEffectStacking {
    // Lets multiple of the same status exist on the same entity
    Stacks,
    // Adds to the timer of the status if it already exists on the entity
    Adds,
    // Refreshes the duration of the status if it already exists on the entity
    Refreshes,
}

#[derive(Clone, Copy)]
pub struct StatusEffect {
    pub status_type: StatusEffectType,
    pub duration: Option<u32>,
    pub stat_modification: Option<StatModification>,
}
impl StatusEffect {
    pub fn priority(&self) -> u8 {
        if let Some(modification) = self.stat_modification {
            modification.priority()
        }
        else {
            0
        }
    }
}

// Events
#[derive(Clone, Copy)]
pub struct StatusEffectEvent {
    pub effect: StatusEffect,
    pub stacking: StatusEffectStacking,
    pub entity: Entity,
}

#[derive(Clone, Copy)]
pub struct RemoveStatusEffectEvent {
    pub entity: Entity,
}