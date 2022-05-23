use crate::{actors::{stats::{Stats, StatChangeEvent, StatType}}, data::{Position}, log::Log};
use bevy::prelude::*;
use rand::Rng;
use caith::{Roller, RollResultType};

extern crate strfmt;
use strfmt::strfmt;
use std::collections::{HashMap, VecDeque};

use super::movement::Collidables;

// Events
pub struct BumpEvent {
    pub bumping_entity: Entity,
    pub bumped_entity: Entity,
}

pub struct RandRangedAttackEvent {
    pub targetting_entity: Entity,
    pub target: IVec2,
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

pub fn ranged_attack (
    // This may become something else later but it will do for now.
    mut ev_ranged_attack_event: EventReader<RandRangedAttackEvent>,
    mut ev_stat_change: EventWriter<StatChangeEvent>,

    name_query: Query<&Name>,
    attacker_query: Query<&RangedAttacker>,
    position_query: Query<&Position>,
    mut stats_query: Query<&mut Stats>,

    mut log: ResMut<Log>,
    collidables: Res<Collidables>,
) {
    // Lets program this first, and then generalize similarities between ranged attacks and melee attacks later.
    // We also want to make it so that ranged/melee attacks can be selected manually, but we can also do this later too.

    // TODO: generalize similarities between melee and ranged and place into different system(s)
    // TODO: add dodge roll after generalization
    for ev in ev_ranged_attack_event.iter() {
        if let Ok(attacker_comp) = attacker_query.get(ev.targetting_entity) {
            println!("hngghh i have the capacity to attack!!!");
            let mut rng = rand::thread_rng();

            let mut attacker_name = ev.targetting_entity.id().to_string();
            if let Ok(name) = name_query.get(ev.targetting_entity) {
                attacker_name = name.to_string();
            }

            // TODO: We need to get this info when we hit an entity.
            //let mut attacked_name = ev.target_entity.id().to_string();
            //if let Ok(name) = name_query.get(ev.target_entity) {
            //    attacked_name = name.to_string();
            //}

            let mut remaining_projectiles = attacker_comp.projectiles.clone();
            let mut projectile_index: usize;
            let mut projectile: Projectile;

            let mut projectile_valid = false;
            let mut has_cost = false;
            let mut can_pay = false;
            
            // Look and find a projectile we can do and do it.
            'projectiles: while remaining_projectiles.len() != 0 {
                projectile_index = rng.gen_range(0..remaining_projectiles.len());
                projectile = remaining_projectiles.swap_remove(projectile_index);

                projectile.attack.damage.roll();
                projectile.attack.cost.roll();

                has_cost = projectile.attack.cost.total != 0;

                if has_cost {
                    if let Ok(stats_attacker) = stats_query.get(ev.targetting_entity) {
                        can_pay = stats_attacker.0.contains_key(&projectile.attack.cost_type) && 
                                  stats_attacker.0[&projectile.attack.cost_type].value + projectile.attack.cost.total > 0;
                    }
                }
                
                // TODO: automatically select an attack that will be effective. have some fallback if none apply
                // TODO: pay cost here if needed




                
                if let Ok(stats_attacker) = stats_query.get(ev.targetting_entity) {
                    if has_cost == can_pay {
                        println!("firing projectile(s)!");

                        if has_cost {
                            // TODO: Deduct cost using stat change event

                        }

                        // Burst fire. TODO: Implement blast fire too.
                        for _ in 0..projectile.count {
                            let d20 = Dice::new("1d20");
                            let roll = d20.total + stats_attacker.get_value(&projectile.spread_save_type);
                            let penalty = (roll - projectile.spread_save) as f32 * projectile.spread_penalty;
                            let spread = if penalty < 0.0 {projectile.optimal_spread + penalty} else {projectile.optimal_spread};
                            // TODO: generate angle sexer
                            let angle = rng.gen_range(-spread as i32..= spread as i32) as f32;

                            let targetting_position = position_query.get(ev.targetting_entity).unwrap().as_vec2();
                            let target_position = rotate_point(targetting_position, ev.target.as_vec2(), angle);
                            
                            let d20 = Dice::new("1d20");
                            let roll = d20.total + stats_attacker.get_value(&projectile.range_save_type);
                            let penalty = (roll - projectile.range_save) as f32 * projectile.range_penalty;
                            let length = projectile.optimal_range - penalty;

                            let mut line_points = get_line_points(targetting_position, target_position, length);
                            // Remove first point. Later, we are also going to want to ignore the first few points that contain friends. (But still hit enemies)
                            // TODO: we need to ignore ALL allies within the first 3 or so tiles, not just the attacker themself. We cannot do this through removing points, however.
                            line_points.pop_front();

                            
                            

                            for point in line_points {
                                if let Some(collided_entity) = collidables[point] {
                                    let mut attacked_name = collided_entity.id().to_string();
                                    if let Ok(name) = name_query.get(collided_entity) {
                                        attacked_name = name.to_string();
                                    }
                                    println!("shooty hit a {}", attacked_name);
                                    // Do this if our projectile hits.
                                    /*
                                    ev_stat_change.send(StatChangeEvent{stat: attack.damage_type, amount: attack.damage.total, entity: ev.bumped_entity});

                                    let vars = HashMap::from([
                                        ("attacker".to_string(), attacker_name),
                                        ("attacked".to_string(), attacked_name),
                                        ("amount".to_string(), attack.damage.total.to_string()),
                                    ]);

                                    let text_index = rng.gen_range(0..attack.interact_text.len());
                                    log.log_string_formatted(format![" {}", strfmt(&attack.interact_text[text_index], &vars).unwrap()], Color::RED);
                                    // 0.2.0 TODO: Add verbose dice rolls resource (bool)
                                    //             Log verbose dice rolls if resource is true
                                    */

                                    // TODO: Log something special if we miss our target or if we hit something besides our intended target.

                                    break 'projectiles;
                                }
                            }
                        }
                    }
                }  
            }
        }
    }
}



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





// Helper functions
fn rotate_point(pivot: Vec2, point: Vec2, rotation: f32) -> Vec2 {
    let sin = rotation.sin();
    let cos = rotation.cos();

    Vec2::new(cos * (point.x - pivot.x) - sin * (point.y - pivot.y) + point.x,
              sin * (point.x - pivot.x) - cos * (point.y - pivot.y) + point.y)
}

fn get_line_points(point_a: Vec2, point_b: Vec2, distance: f32) -> VecDeque<IVec2> {
    let mut points = VecDeque::new();
    //let distance = point_a.distance(point_b);
    for step in 0..=distance as i32 {
        let s = if distance == 0.0 {0.0} else {step as f32 / distance};
        points.push_back(point_a.lerp(point_b, s).round().as_ivec2());
    }

    points
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

/// Should be fine? TODO: Remove uncertainty if this works fine.  
/// We can make count into a Dice later if we need to. (Variable amount of projectiles per ranged attack)  
/// Blast tells us if all of the shots come out at once or as separate shots. Basically shotgun vs smg.  
///     If we blast, all the bullets will be spread in the same direction by the penalty, and then each individual pellet will be modified by the optimal spread.  
///     If we burst, all bullets will be randomly spread out by optimal spread minus.  
#[derive(Clone)]
pub struct Projectile {
    pub attack: Attack,
    pub count: u32,
    pub blast: bool,

    pub spread_save: i32,
    pub spread_save_type: StatType,
    pub optimal_spread: f32,
    pub spread_penalty: f32,

    pub range_save: i32,
    pub range_save_type: StatType,
    pub optimal_range: f32,
    pub range_penalty: f32,         

}
impl Default for Projectile {
    fn default() -> Self {
        Projectile {
            attack: Attack::default(),
            count: 1,
            blast: false,

            spread_save: 10,
            spread_save_type: StatType::Dexterity,
            optimal_spread: 0.0_f32.to_radians(),
            spread_penalty: 4.0_f32.to_radians(),

            range_save: 10,
            range_save_type: StatType::Dexterity,
            optimal_range: 10.0,
            range_penalty: 1.0,         
        }
    }

}

// Components
#[derive(Component, Default, Clone)]
pub struct MeleeAttacker {
    pub attacks: Vec<Attack>,
}

#[derive(Component, Default, Clone)]
pub struct RangedAttacker {
    pub projectiles: Vec<Projectile>,
}



