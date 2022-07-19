use bevy::prelude::*;
use std::fmt::Display;

use crate::{turn::Turns, ai::targetting_behavior::Engages};

use super::{TakesTurns, stats::{StatModification, StatType}};

// Systems
// I don't know if I want to move this to be with the rest of the behaviors or not.
// It may not only affect AI.
pub fn tranced_behavior (
    mut turns: ResMut<Turns>,

    tranced_query: Query<&StatusEffects, With<TakesTurns>>,
) {
    // TODO: Maybe we should turn this into a system condition?
    if turns.progress == true {
        return;
    }

    let entity = turns.order[turns.current];
    if let Ok(statuses) = tranced_query.get(entity) {
        if statuses.has_status_effect(&StatusEffectType::Tranced) {
            println!("wuhh im tranced woah");
            turns.progress_turn();
        }
    }
}

pub fn cumblobbed_behavior (
    mut turns: ResMut<Turns>,

    mut tranced_query: Query<(&StatusEffects, Option<&mut Engages>), With<TakesTurns>>,
) {
    // TODO: Maybe we should turn this into a system condition?
    if turns.progress == true {
        return;
    }

    let entity = turns.order[turns.current];
    if let Ok((statuses, mut opt_engagement)) = tranced_query.get_mut(entity) {
        if let Some(from) = statuses.status_from(&&StatusEffectType::Cumblobbed) {
            if let Some(mut engagement) = opt_engagement {
                engagement.delay_timer = engagement.delay;
                engagement.target = from;
            }
            println!("wuhh im cumblobbed woah");
            turns.progress_turn();
        }
    }
}

pub fn update_status_effects (
    turns: Res<Turns>,

    mut status_query: Query<(Entity, &mut StatusEffects)>,

    mut ev_removed_status_effect: EventWriter<RemoveStatusEffectEvent>,
) {
    if turns.progress {
        for (entity, mut statuses) in status_query.iter_mut() {
            if turns.was_turn(&entity) {
                let length = statuses.len();
                for status in statuses.iter_mut() {
                    if let Some(duration) = status.duration.as_mut() {
                        *duration -= 1;
                    }
                }
                statuses.retain(|a| a.duration != Some(0));
    
                if statuses.len() != length {
                    ev_removed_status_effect.send(RemoveStatusEffectEvent{entity});
                }
            }
        }
    }
}

pub fn apply_status_effects (
    mut ev_status_effect: EventReader<StatusEffectEvent>,

    mut status_query: Query<&mut StatusEffects>,
) {
    for ev in ev_status_effect.iter() {
        let mut append = true;

        if let Ok(mut statuses) = status_query.get_mut(ev.entity) {
            if !matches!(ev.application.stacking, StatusEffectStacking::Stacks) {
                for status in statuses.iter_mut() {
                    if status.status_type == ev.application.effect.status_type {
                        append = false;
                        if matches!(ev.application.stacking, StatusEffectStacking::Adds) {
                            if let Some(duration) = status.duration.as_mut() {
                                if let Some(ev_duration) = ev.application.effect.duration {
                                    *duration += ev_duration;
                                }
                                else {
                                    status.duration = None;
                                }
                            }
                        }
                        else {
                            status.duration = ev.application.effect.duration;
                        }
                    }
                }
            }
        }

        if append {
            if let Ok(mut statuses) = status_query.get_mut(ev.entity) {
                println!("applying {:?}", ev.application.effect.status_type);
                statuses.push(ev.application.effect);

                statuses.sort_by(|a, b| a.priority().cmp(&b.priority()))
            }
        }
    }
}

// Components
#[derive(Component, Clone, Default, Deref, DerefMut, Debug)]
pub struct StatusEffects (Vec<StatusEffect>);
impl StatusEffects {
    pub fn has_status_effect(&self, status_effect_type: &StatusEffectType) -> bool {
        for status in self.iter() {
            if status.status_type == *status_effect_type {
                return true
            }
        }

        false
    }

    pub fn status_from(&self, status_effect_type: &StatusEffectType) -> Option<Option<Entity>> {
        for status in self.iter() {
            if status.status_type == *status_effect_type {
                return Some(status.from)
            }
        }

        None
    }
}

// Data
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
#[derive(Clone, Copy, Debug)]
pub enum StatusEffectStacking {
    // Lets multiple of the same status exist on the same entity
    Stacks,
    // Adds to the timer of the status if it already exists on the entity
    Adds,
    // Refreshes the duration of the status if it already exists on the entity
    Refreshes,
}

#[derive(Clone, Copy, Debug)]
pub struct TileModification {
    pub glyph: Option<char>,
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
}

#[derive(Clone, Copy, Debug)]
pub struct StatusEffect {
    pub status_type: StatusEffectType,
    pub from: Option<Entity>,
    pub tile_modification: Option<TileModification>,
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

#[derive(Clone, Copy, Debug)]
pub struct StatusEffectApplication {
    pub effect: StatusEffect,
    pub stacking: StatusEffectStacking,
}

// Events
#[derive(Clone, Copy)]
pub struct StatusEffectEvent {
    pub application: StatusEffectApplication,
    pub entity: Entity,
}

#[derive(Clone, Copy)]
pub struct RemoveStatusEffectEvent {
    pub entity: Entity,
}