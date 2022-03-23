use crate::actors::{MeleeAttacker, Stats};
use bevy::prelude::*;
use rand::Rng;

// Events
pub struct BumpEvent {
    pub bumping_entity: Entity,
    pub bumped_entity: Entity,
}

// Systems
pub fn melee_attack (
    mut ev_bump_event: EventReader<BumpEvent>,

    mut attacker_query: Query<(&MeleeAttacker, Option<&mut Stats>)>,
    mut attackable_query: Query<(&mut Stats)>,
) {
    for ev in ev_bump_event.iter() {
        if let Ok((attacker_comp, mut opt_attacker_stats)) = attacker_query.get_mut(ev.bumping_entity) {
            if let Ok(mut attacked_stats) = attackable_query.get_mut(ev.bumped_entity) {
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
    }
}