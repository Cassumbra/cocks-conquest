use crate::actors::{MeleeAttacker, Stats, Attack};
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

// Systems
pub fn melee_attack (
    mut ev_bump_event: EventReader<BumpEvent>,

    name_query: Query<&Name>,
    attacker_query: Query<&MeleeAttacker>,
    mut stats_query: Query<&mut Stats>,
) {
    for ev in ev_bump_event.iter() {
        
        // Setup stuff

        // Go into loop, check random attacks
        // Check if there is a cost
        //  Check if cost is able to be paid
        // Check if attack type matches with a stat the attacked has

        // if bumping guy has ability to attack (also get the ability to attack as a thingy)

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
                                *stats_attacked.0.get_mut(&attack.damage_type).unwrap() += attack.damage;

                                let vars = HashMap::from([
                                    ("attacker".to_string(), attacker_name),
                                    ("attacked".to_string(), attacked_name),
                                    ("amount".to_string(), attack.damage.to_string()),
                                ]);

                                let text_index = rng.gen_range(0..attack.interact_text.len());
                                println!("{}", strfmt(&attack.interact_text[text_index], &vars).unwrap());
                            }

                            //attack.interact_text[text_index]), &["fuuuck"] 

                            if has_cost {

                            }
                            break;
                        }       
                    }                    
                }
            }
        }

        /*
        if let Ok(attacker_comp) = attacker_query.get_mut(ev.bumping_entity) {
            if let Ok(mut attacked_stats) = stats_query.get_mut(ev.bumped_entity) {
                let attacker = ev.bumping_entity;
                let attacked = ev.bumped_entity;

                let mut costly_attack = false;

                let mut rng = rand::thread_rng();

                let mut remaining_attacks = attacker_comp.attacks.clone();
                let mut attack_index = rng.gen_range(0..remaining_attacks.len());



                while remaining_attacks.len() != 0 {
                    let attack = &remaining_attacks[attack_index];

                    // Check if the attacked is affected by the attack.
                    if attacked_stats.0.contains_key(&attack.damage_type) {
                        // Check if there is a cost for the attack.
                        if let Some((cost, cost_type)) = &attack.cost {
                            if let Some(attacker_stats) = &opt_attacker_stats {
                                if attacker_stats.0.contains_key(cost_type) {
                                    costly_attack = true;
                                }
                            }
                        }
                        // Apply attack.
                        if let Some(stat) = attacked_stats.0.get_mut(&attack.damage_type) {
                            *stat += attack.damage;
                        }
                        break;
                    }

                    // Need to actually do the other stuff here oops i forgor
                }

                // Apply attack cost, if necessary.
                if costly_attack {
                    let attack = &remaining_attacks[attack_index];
                    
                    if let Some((cost, cost_type)) = &attack.cost {
                        if let Some(mut attacker_stats) = opt_attacker_stats {
                            if let Some(stat) = attacker_stats.0.get_mut(cost_type) {
                                *stat += cost;
                            }
                        }
                    }
                }
            }
        }
        */
    }
}