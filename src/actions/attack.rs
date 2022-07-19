use std::collections::HashMap;

use crate::{actors::{stats::{StatType, StatChangeEvent, Stats}, status_effects::{StatusEffectApplication, StatusEffectEvent}}, log::Log};
use bevy::prelude::*;
use caith::{Roller, RollResultType};
use rand::Rng;
use strfmt::strfmt;

extern crate strfmt;



// Events
// TODO: Move this to melee?
pub struct BumpEvent {
    pub bumping_entity: Entity,
    pub bumped_entity: Entity,
}

pub struct AttackEvent {
    pub attacking_entity: Entity,
    pub attacked_entity: Entity,
    // TODO: Should this be an index?
    pub attack: Attack,
    pub attack_type: AttackType,
}

// Systems
pub fn check_attack_cost (

) {

}

pub fn attack_hit (
    mut ev_stat_change: EventWriter<StatChangeEvent>,
    mut ev_status_effect: EventWriter<StatusEffectEvent>,
    mut ev_attack_hit: EventReader<AttackEvent>,

    name_query: Query<&Name>,
    stats_query: Query<&Stats>,

    mut log: ResMut<Log>,
) {
    let mut rng = rand::thread_rng();

    for ev in ev_attack_hit.iter() {

        let mut attack_valid = false;
        let mut has_cost = false;
        let mut can_pay = false;

        let mut attack = ev.attack.clone();

        attack.cost.roll();
        attack.damage.roll();

        has_cost = attack.cost.total != 0;

        if has_cost {
            if let Ok(stats_attacker) = stats_query.get(ev.attacking_entity) {
                can_pay = stats_attacker.0.contains_key(&attack.cost_type) && 
                          stats_attacker.0[&attack.cost_type].effective + attack.cost.total > 0;

                if !can_pay {
                    println!("Cannot pay cost of attack!");
                    return;
                } else {
                    // TODO: Pay cost of attack here.
                }
            }
        }

        if let Ok(stats_attacked) = stats_query.get(ev.attacked_entity) {
            let value_to_be = stats_attacked.get_effective(&attack.damage_type) + attack.damage.total;

            if stats_attacked.0.contains_key(&attack.damage_type) {
                attack_valid = attack.damage.total < 0 && stats_attacked.get_effective(&attack.damage_type) > stats_attacked.get_min(&attack.damage_type) ||
                               attack.damage.total > 0 && stats_attacked.get_effective(&attack.damage_type) < stats_attacked.get_max(&attack.damage_type);
                println!("attack valid: {}", attack_valid);
            }
        }

        if attack_valid {
            let mut attacker_name = ev.attacking_entity.id().to_string();
            if let Ok(name) = name_query.get(ev.attacking_entity) {
                attacker_name = name.to_string();
            }

            let mut attacked_name = ev.attacked_entity.id().to_string();
            if let Ok(name) = name_query.get(ev.attacked_entity) {
                attacked_name = name.to_string();
            }

            let vars = HashMap::from([
                ("attacker".to_string(), attacker_name),
                ("attacked".to_string(), attacked_name),
                ("amount".to_string(), attack.damage.total.to_string()),
            ]);

            // TODO: Add capacity for target to dodge/resist attack.

            ev_stat_change.send(StatChangeEvent{stat: attack.damage_type, amount: attack.damage.total, entity: ev.attacked_entity});
            if let Some(application) = attack.status_effect {
                // TODO: This is silly, can we not implement some way of changing "from" before we get to this point?
                let mut a = application;
                a.effect.from = Some(ev.attacking_entity);
                ev_status_effect.send(StatusEffectEvent { application: a, entity: ev.attacked_entity });
            }
            
        
            let text_index = rng.gen_range(0..attack.interact_text.len());
            log.log_string_formatted(format![" {}", strfmt(&attack.interact_text[text_index], &vars).unwrap()], Color::RED);
            // 0.2.0 TODO: Add verbose dice rolls resource (bool)
            //             Log verbose dice rolls if resource is true
            // TODO: Log something special if we miss our target or if we hit something besides our intended target.

        }
        else {
            // Uhhhhh idk lol
        }
    }
}

// Misc Data
#[derive(Clone)]
pub struct Dice {
    pub expression: String,
    pub reason: String,
    pub total: i32,
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
    pub damage_type: StatType,
    pub cost: Dice,
    pub cost_type: StatType,
    pub save_text: Vec<String>,
    pub save: i32,
    pub save_type: StatType,
    pub status_effect: Option<(StatusEffectApplication)>,
}
impl Default for Attack {
    fn default() -> Attack {
        Attack {
            interact_text: vec![String::from("{attacker} hits {attacked} for {amount} damage!")],
            damage: Dice::new("1d4 * -1"),
            damage_type: StatType::Health,
            cost: Dice::new("0"),
            cost_type: StatType::Health,
            save_text: vec![String::from("{attacked} dodges {attacker}'s attack!")],
            save: 16,
            save_type: StatType::Dexterity,
            status_effect: None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum AttackType {
    Ranged,
    Melee,
    Digestion,
}