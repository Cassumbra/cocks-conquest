use crate::{actors::{stats::StatType}};
use bevy::prelude::*;
use caith::{Roller, RollResultType};

extern crate strfmt;



// Events
pub struct BumpEvent {
    pub bumping_entity: Entity,
    pub bumped_entity: Entity,
}



pub struct ActorRemovedEvent {
    pub removed_actor: Entity,
}



// Systems
pub fn check_attack_cost (

) {

}

pub fn do_attack (

) {

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
        }
    }
}