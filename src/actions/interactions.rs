use crate::{actors::{stats::{Stats, StatChangeEvent, Tranced}, TakesTurns}, data::{Position, Collides}, rendering::Renderable, turn::Turns, log::Log};
use bevy::prelude::*;
use rand::Rng;
use caith::{Roller, RollResult, RollResultType, RepeatedRollResult, SingleRollResult};

extern crate strfmt;
use strfmt::strfmt;
use std::collections::HashMap;

// Events
pub struct BumpEvent {
    pub bumping_entity: Entity,
    pub bumped_entity: Entity,
}

pub struct ActorRemovedEvent {
    pub removed_actor: Entity,
}

pub struct HealActionEvent{
    pub healing_entity: Entity,
}

// Systems
pub fn melee_attack (
    mut ev_bump_event: EventReader<BumpEvent>,
    mut ev_stat_change: EventWriter<StatChangeEvent>,

    name_query: Query<&Name>,
    attacker_query: Query<&MeleeAttacker>,
    mut stats_query: Query<&mut Stats>,

    mut log: ResMut<Log>,
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
                                      stats_attacker.0[&attack.cost_type].value + attack.cost.total > 0;
                        }
                    }
                    
                    if let Ok(stats_attacked) = stats_query.get_mut(ev.bumped_entity) {
                        let value_to_be = stats_attacked.get_value(&attack.damage_type) + attack.damage.total;

                        if stats_attacked.0.contains_key(&attack.damage_type) {
                            attack_valid = attack.damage.total < 0 && stats_attacked.get_value(&attack.damage_type) > stats_attacked.get_min(&attack.damage_type) ||
                                           attack.damage.total > 0 && stats_attacked.get_value(&attack.damage_type) < stats_attacked.get_max(&attack.damage_type);
                            println!("attack valid: {}", attack_valid);
                        }

                        if attack_valid && (has_cost == can_pay) {
                            println!("doing attack!");

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
                            break;
                        }       
                    }                    
                }
            }
        }
    }
}

pub fn ranged_attack (

) {
    
}

pub fn vore_attack(
    mut commands: Commands,

    mut ev_bump_event: EventReader<BumpEvent>,

    prey_query: Query<(&Tranced, Option<&Name>)>,
    pred_query: Query<(&DoesVore, Option<&Name>)>,

    mut log: ResMut<Log>,
) {
    // TODO: print stuff to log (once we make one)

    for ev in ev_bump_event.iter() {
        if let Ok((_tranced, opt_prey_name)) = prey_query.get(ev.bumped_entity) {
            if let Ok((_doesvore, opt_pred_name)) = pred_query.get(ev.bumping_entity) {
                let prey_name = if opt_prey_name.is_some() {opt_prey_name.unwrap().to_string()} else {ev.bumped_entity.id().to_string()};
                let pred_name = if opt_pred_name.is_some() {opt_pred_name.unwrap().to_string()} else {ev.bumping_entity.id().to_string()};
    
                log.log_string_formatted(format!(" {} devours {}!", pred_name, prey_name), Color::RED);
                commands.entity(ev.bumped_entity)
                    .remove::<Collides>()
                    .remove::<Renderable>()
                    .remove::<TakesTurns>()
                    .insert(Digesting{
                        turns_to_digest: 4,
                    });
                commands.entity(ev.bumping_entity)
                    .push_children(&[ev.bumped_entity]);
            }
        }
    }
}

pub fn update_vore (
    mut commands: Commands,

    mut prey_query: Query<(&mut Digesting, Option<&Name>)>,
    mut pred_query: Query<(Entity, &mut Stats, Option<&Name>, &Children), With<TakesTurns>>,

    mut ev_actor_remove_event: EventWriter<ActorRemovedEvent>,

    turns: Res<Turns>,
    mut log: ResMut<Log>,
) {
    for (pred, mut stats, opt_pred_name, meals) in pred_query.iter_mut() {
        
        if turns.was_turn(&pred) {
            for prey in meals.iter() {
                if let Ok((mut digestion, opt_prey_name)) = prey_query.get_mut(*prey) {
                    let prey_name = if opt_prey_name.is_some() {opt_prey_name.unwrap().to_string()} else {prey.id().to_string()};
                    let pred_name = if opt_pred_name.is_some() {opt_pred_name.unwrap().to_string()} else {pred.id().to_string()};

                    digestion.turns_to_digest -= 1;
                    if digestion.turns_to_digest == 0 {
                        // TODO: Add a check to make sure we don't go over the limit
                        log.log_string_formatted(format!(" {} has been melted into 15 cum points worth of stinky smelly goo.", prey_name), Color::GREEN);
                        commands.entity(*prey).despawn();
                        ev_actor_remove_event.send(ActorRemovedEvent{removed_actor: *prey});
                        stats.0.get_mut("cum points").unwrap().value += 15;
                    } else {
                        log.log_string_formatted(format!(" {} turns until {} is digested by {}.", digestion.turns_to_digest, prey_name, pred_name), Color::WHITE);
                    }
                } 

            }
        }
    }
}

pub fn heal_action (
    mut heal_query: Query<(&mut Stats, Option<&Name>), With<CanHeal>>,

    mut ev_heal_event: EventReader<HealActionEvent>,

    mut log: ResMut<Log>,
) {
    for ev in ev_heal_event.iter() {
        if let Ok((mut stats, opt_name)) = heal_query.get_mut(ev.healing_entity) {
            let name = if opt_name.is_some() {opt_name.unwrap().to_string()} else {ev.healing_entity.id().to_string()};

            // Both of these should be retrieved dynamically from the CanHeal component and/or a MaxStats component in the future.
            if stats.0.get("cum points").unwrap().value >= 5 && stats.0.get("health").unwrap().value < stats.get_max("health") {
                log.log_string_formatted(format!(" {} uses 5 cum points to heal for 1 health.", name), Color::GREEN);
                stats.0.get_mut("cum points").unwrap().value -= 5;
                stats.0.get_mut("health").unwrap().value += 1;
            }
        }
    }

}

// Misc Data
#[derive(Clone)]
pub struct Dice {
    expression: String,
    reason: String,
    total: i32,
}
impl Dice {
    pub fn new(expression: &str) -> Dice {
        let (reason, total) = Dice::parse_string(&expression);
        Dice { expression: String::from(expression), reason, total }
    }

    pub fn roll(&mut self) -> (String, i32) {
        let (reason, total) = Dice::parse_string(&self.expression);
        self.reason = reason.clone();
        self.total = total;

        println!("rolling the dice: {}", total);

        (reason, total)
    }

    // This is where the magic happens.
    // If we get pissed about the code here, we can change it without it negatively effecting the rest of the program.
    /// Returns a reason for a result and the result itself.
    fn parse_string(string: &str) -> (String, i32) {
        let roll = Roller::new(string).unwrap().roll().unwrap();

        match roll.get_result() {
            RollResultType::Single(result) => {
                (format!("{}", roll), (result.get_total()).try_into().unwrap())
            }
            RollResultType::Repeated(result) => {
                (format!("{}", roll), result.get_total().unwrap().try_into().unwrap())
            }
        }
    }
}

#[derive(Clone)]
pub struct Attack {
    pub interact_text: Vec<String>,
    pub damage: Dice,
    pub damage_type: String,
    pub cost: Dice,
    pub cost_type: String,
    pub save: i32,
    pub save_type: String,
}
impl Default for Attack {
    fn default() -> Attack {
        Attack {
            interact_text: vec![String::from("{attacker} hits {attacked} for {amount} damage!")],
            damage: Dice::new("1d4 * -1"),
            damage_type: String::from("health"),
            cost: Dice::new("0"),
            cost_type: String::from("health"),
            save: 16,
            save_type: String::from("dexterity"),
        }
    }
}

//#[derive(Clone)]
//pub struct RangedAttack {
//    pub attack: Attack,
//    pub range: 
//}

// Components
#[derive(Component, Default, Clone)]
pub struct MeleeAttacker {
    pub attacks: Vec<Attack>,
}

// Should be dynamic like how attacks are (at some point but i don't care)
#[derive(Component, Default, Copy, Clone)]
pub struct DoesVore;

#[derive(Component, Default, Copy, Clone)]
pub struct Digesting {
    pub turns_to_digest: u8,
}

// Should be dynamic like how attacks are (at some point but i dont care)
#[derive(Component, Default, Copy, Clone)]
pub struct CanHeal;