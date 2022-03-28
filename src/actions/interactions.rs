use crate::{actors::{stats::{Stats, StatChangeEvent, Tranced}, TakesTurns}, components::{Position, Collides}, rendering::Renderable, turn::Turns};
use bevy::prelude::*;
use rand::Rng;

extern crate strfmt;
use strfmt::strfmt;
use std::collections::HashMap;

// Events
pub struct BumpEvent {
    pub bumping_entity: Entity,
    pub bumped_entity: Entity,
}

pub struct ActorRemovedEvent;

// Systems
pub fn melee_attack (
    mut ev_bump_event: EventReader<BumpEvent>,
    mut ev_stat_change: EventWriter<StatChangeEvent>,

    name_query: Query<&Name>,
    attacker_query: Query<&MeleeAttacker>,
    mut stats_query: Query<&mut Stats>,
) {
    for ev in ev_bump_event.iter() {
        if let Ok(attacker_comp) = attacker_query.get(ev.bumping_entity) {
            if stats_query.get(ev.bumped_entity).is_ok() {
                let mut rng = rand::thread_rng();

                let mut attacker_name = ev.bumping_entity.id().to_string();
                if let Ok(name) = name_query.get(ev.bumping_entity) {
                    attacker_name = name.to_string();
                }

                let mut attacked_name = ev.bumped_entity.id().to_string();
                if let Ok(name) = name_query.get(ev.bumped_entity) {
                    attacked_name = name.to_string();
                }

                let mut remaining_attacks = attacker_comp.attacks.clone();
                let mut attack_index: usize;
                let mut attack: Attack;

                let mut attack_valid = false;
                let mut has_cost = false;
                let mut can_pay = false;
                

                while remaining_attacks.len() != 0 {
                    attack_index = rng.gen_range(0..remaining_attacks.len());
                    attack = remaining_attacks.swap_remove(attack_index);

                    has_cost = attack.cost != 0;

                    if has_cost {
                        if let Ok(stats_attacker) = stats_query.get(ev.bumping_entity) {
                            can_pay = stats_attacker.0.contains_key(&attack.cost_type) && 
                                      stats_attacker.0[&attack.cost_type] + attack.cost > 0;
                        }
                    }
                    
                    if let Ok(stats_attacked) = stats_query.get(ev.bumped_entity) {
                        attack_valid = stats_attacked.0.contains_key(&attack.damage_type);

                        if attack_valid && (has_cost == can_pay) {
                            if let Ok(mut stats_attacked) = stats_query.get_mut(ev.bumped_entity) {
                                ev_stat_change.send(StatChangeEvent{stat: attack.damage_type, amount: attack.damage, entity: ev.bumped_entity});
                                //*stats_attacked.0.get_mut(&attack.damage_type).unwrap() += attack.damage;

                                let vars = HashMap::from([
                                    ("attacker".to_string(), attacker_name),
                                    ("attacked".to_string(), attacked_name),
                                    ("amount".to_string(), attack.damage.to_string()),
                                ]);

                                let text_index = rng.gen_range(0..attack.interact_text.len());
                                println!("{}", strfmt(&attack.interact_text[text_index], &vars).unwrap());
                            }

                            if has_cost {
                                // TODO: IMPLEMENT THIS

                            }
                            break;
                        }       
                    }                    
                }
            }
        }
    }
}

pub fn vore_attack(
    mut commands: Commands,

    mut ev_bump_event: EventReader<BumpEvent>,

    prey_query: Query<&Tranced>,
    pred_query: Query<&DoesVore>,
) {
    // TODO: print stuff to log (once we make one)

    for ev in ev_bump_event.iter() {
        if prey_query.get(ev.bumped_entity).is_ok() &&
           pred_query.get(ev.bumping_entity).is_ok()
        {
            commands.entity(ev.bumped_entity)
                .remove::<Collides>()
                .remove::<Renderable>()
                .remove::<TakesTurns>()
                .insert(Digesting{
                    turns_to_digest: 3,
                });
            commands.entity(ev.bumping_entity)
                .push_children(&[ev.bumped_entity]);
        }
    }
}

pub fn update_vore (
    mut commands: Commands,

    mut prey_query: Query<(&mut Digesting)>,
    mut pred_query: Query<(Entity, &mut Stats, &Children), With<TakesTurns>>,

    mut ev_actor_remove_event: EventWriter<ActorRemovedEvent>,

    turns: Res<Turns>,
) {
    // TODO: print stuff to log (once we make one)

    for (pred, mut stats, prey) in pred_query.iter_mut() {
        
        if turns.was_turn(&pred) {
            for p in prey.iter() {
                if let Ok(mut digestion) = prey_query.get_mut(*p) {
                    digestion.turns_to_digest -= 1;
                    if digestion.turns_to_digest == 0 {
                        commands.entity(*p).despawn();
                        ev_actor_remove_event.send(ActorRemovedEvent);
                        *stats.0.get_mut("cum points").unwrap() += 15;
                    }
                } 

            }
        }
    }


}

// Misc Data
#[derive(Clone)]
pub struct Attack {
    pub interact_text: Vec<String>,
    pub damage: i32,
    pub damage_type: String,
    pub cost: i32,
    pub cost_type: String,
}
impl Default for Attack {
    fn default() -> Attack {
        Attack {
            interact_text: vec!["{attacker} hits {attacked} for {amount} damage!".to_string()],
            damage: 1,
            damage_type: "health".to_string(),
            cost: 0,
            cost_type: "health".to_string(),
        }
    }
}



// Components
#[derive(Component, Default, Clone)]
pub struct MeleeAttacker {
    pub attacks: Vec<Attack>,
}

#[derive(Component, Default, Copy, Clone)]
pub struct DoesVore;

#[derive(Component, Default, Copy, Clone)]
pub struct Digesting {
    pub turns_to_digest: u8,
}