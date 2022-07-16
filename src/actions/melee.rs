use std::collections::HashMap;

use bevy::{prelude::*};
use rand::Rng;
use strfmt::strfmt;

use crate::{actors::stats::{StatChangeEvent, Stats}, log::Log};
use super::attack::{Attack, BumpEvent, AttackEvent};


// Components 
#[derive(Component, Default, Clone)]
pub struct MeleeAttacker {
    pub attacks: Vec<Attack>,
}

// Systems
pub fn bump_melee_attack (
    mut ev_bump_event: EventReader<BumpEvent>,
    mut ev_attack_hit: EventWriter<AttackEvent>,

    name_query: Query<&Name>,
    attacker_query: Query<&MeleeAttacker>,
    mut stats_query: Query<&mut Stats>,

    mut log: ResMut<Log>,
) {
    let mut rng = rand::thread_rng();

    for ev in ev_bump_event.iter() {
        if let Ok(attacker_comp) = attacker_query.get(ev.bumping_entity) {
            if stats_query.get(ev.bumped_entity).is_ok() {
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
                
                // Look and find an attack we can do and do it.
                while remaining_attacks.len() != 0 {
                    attack_index = rng.gen_range(0..remaining_attacks.len());
                    attack = remaining_attacks.swap_remove(attack_index);

                    attack.damage.roll();
                    attack.cost.roll();

                    has_cost = attack.cost.total != 0;

                    if has_cost {
                        if let Ok(stats_attacker) = stats_query.get(ev.bumping_entity) {
                            can_pay = stats_attacker.0.contains_key(&attack.cost_type) && 
                                      stats_attacker.0[&attack.cost_type].effective + attack.cost.total > 0;
                        }
                    }
                    
                    if let Ok(stats_attacked) = stats_query.get_mut(ev.bumped_entity) {
                        let value_to_be = stats_attacked.get_effective(&attack.damage_type) + attack.damage.total;

                        if stats_attacked.0.contains_key(&attack.damage_type) {
                            attack_valid = attack.damage.total < 0 && stats_attacked.get_effective(&attack.damage_type) > stats_attacked.get_min(&attack.damage_type) ||
                                           attack.damage.total > 0 && stats_attacked.get_effective(&attack.damage_type) < stats_attacked.get_max(&attack.damage_type);
                            println!("attack valid: {}", attack_valid);
                        }

                        if attack_valid && (has_cost == can_pay) {
                            println!("doing melee attack!");

                            ev_attack_hit.send(AttackEvent { attacking_entity: ev.bumping_entity, attacked_entity: ev.bumped_entity, attack });

                            /*
                            ev_stat_change.send(StatChangeEvent{stat: attack.damage_type, amount: attack.damage.total, entity: ev.bumped_entity});
                            //*stats_attacked.0.get_mut(&attack.damage_type).unwrap() += attack.damage;

                            let vars = HashMap::from([
                                ("attacker".to_string(), attacker_name),
                                ("attacked".to_string(), attacked_name),
                                ("amount".to_string(), attack.damage.total.to_string()),
                            ]);

                            let text_index = rng.gen_range(0..attack.interact_text.len());
                            log.log_string_formatted(format![" {}", strfmt(&attack.interact_text[text_index], &vars).unwrap()], Color::RED);
                            // 0.2.0 TODO: Add verbose dice rolls resource (bool)
                            //             Log verbose dice rolls if resource is true
                            

                            if has_cost {
                                // TODO: IMPLEMENT THIS
                                if let Ok(stats_attacker) = stats_query.get_mut(ev.bumping_entity) {

                                }

                            }
                            */ */
                            break;
                        }       
                    }                    
                }
            }
        }
    }
}