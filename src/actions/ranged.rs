use std::{time::Duration, collections::{VecDeque, HashMap}};

use bevy::prelude::*;
use bevy_ascii_terminal::Tile;
use rand::Rng;

use crate::{rendering::effects::{EffectFragment, EffectTile, Effect}, actors::stats::{StatChangeEvent, Stats, StatType}, log::Log, data::Position, actions::attack::Dice};

use super::{attack::{Attack, AttackEvent}, movement::Collidables};

// Events
pub struct RandRangedAttackEvent {
    pub targetting_entity: Entity,
    pub target: IVec2,
}

pub struct RangedAttackEvent {
    pub targetting_entity: Entity,
    pub target: IVec2,
    pub projectile: Projectile,
}

pub struct ProjectileHitEvent {
    pub targetting_entity: Entity,
    pub hit_entity: Entity,
    pub projectile: Projectile,
}

// Systems
pub fn ranged_attack (
    mut ev_ranged_attack: EventReader<RangedAttackEvent>,
    mut ev_attack_hit: EventWriter<AttackEvent>,

    name_query: Query<&Name>,
    attacker_query: Query<&RangedAttacker>,
    position_query: Query<&Position>,
    mut stats_query: Query<&mut Stats>,

    mut commands: Commands,

    mut log: ResMut<Log>,
    collidables: Res<Collidables>,
) {
    'events: for ev in ev_ranged_attack.iter() {
        if let Ok(attacker_comp) = attacker_query.get(ev.targetting_entity) {
            println!("hngghh i have the capacity to attack!!!");
            let mut rng = rand::thread_rng();

            let mut attacker_name = ev.targetting_entity.id().to_string();
            if let Ok(name) = name_query.get(ev.targetting_entity) {
                attacker_name = name.to_string();
            }

            let mut has_cost = false;
            let mut can_pay = false;

            let mut projectile = ev.projectile.clone();

            projectile.attack.damage.roll();
            projectile.attack.cost.roll();

            projectile.cost.roll();

            has_cost = projectile.attack.cost.total != 0;
            
            // TODO: Allow for attacks fired by entities without stats.
            if let Ok(stats_attacker) = stats_query.get(ev.targetting_entity) {
                if has_cost {
                    can_pay = stats_attacker.0.contains_key(&projectile.cost_type) && 
                        stats_attacker.0[&projectile.cost_type].value + projectile.cost.total > 0;

                    if can_pay {
                        // TODO: pay cost
                    }
                    else {
                        // early return if can't pay cost
                        return;
                    }
                }

                
                // fire projectile here

                let mut effect = Effect::default();

                println!("firing projectile(s)!");

                // Burst fire. TODO: Implement blast fire too.
                'burst_fire: for _ in 0..projectile.count {
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
                    let length = if penalty < 0.0 {projectile.optimal_range + penalty} else {projectile.optimal_range};
                    

                    // TODO: Make our projectile path directly to its target avoiding collidables if it can do so in a straight line
                    let mut line_points = get_line_points(targetting_position, target_position, length);
                    // Remove first point. Later, we are also going to want to ignore the first few points that contain friends. (But still hit enemies)
                    // TODO: we need to ignore ALL allies within the first 3 or so tiles, not just the attacker themself. We cannot do this through removing points, however.
                    
                    line_points.pop_front();

                    println!("points in line: {:?}", line_points);
                    

                    for point in line_points {

                        println!("projectile at point {}", point);
                        effect.fragments.push(EffectFragment{duration: Duration::from_secs_f32(0.25), tiles: vec![EffectTile{position: point, tile: Tile { glyph: '-', fg_color: Color::YELLOW, bg_color: Color::BLACK }}]});

                        if let Some(collided_entity) = collidables[point] {
                            let mut attacked_name = collided_entity.id().to_string();
                            if let Ok(name) = name_query.get(collided_entity) {
                                attacked_name = name.to_string();
                            }
                            println!("shooty hit a {}", attacked_name);

                            ev_attack_hit.send(AttackEvent{ attacking_entity: ev.targetting_entity, attacked_entity: collided_entity, attack: projectile.attack.clone() });
                            
                            continue 'burst_fire;
                        }
                    }
                }

                commands.spawn().insert(effect);
            }
        }
    }
}

pub fn rand_ranged_attack (
    mut ev_ranged_attack_event: EventReader<RandRangedAttackEvent>,
    mut ev_stat_change: EventWriter<StatChangeEvent>,

    name_query: Query<&Name>,
    attacker_query: Query<&RangedAttacker>,
    position_query: Query<&Position>,
    mut stats_query: Query<&mut Stats>,

    mut commands: Commands,

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

                let mut effect = Effect::default();


                
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

                            println!("points in line: {:?}", line_points);
                            

                            for point in line_points {

                                println!("projectile at point {}", point);
                                effect.fragments.push(EffectFragment{duration: Duration::from_secs_f32(0.25), tiles: vec![EffectTile{position: point, tile: Tile { glyph: '-', fg_color: Color::YELLOW, bg_color: Color::BLACK }}]});

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

                                    commands.spawn().insert(effect);
                                    // TODO: breaking from projectiles means we dont actually get to fire off more than one projectile. Fix this.
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

// TODO: Create unified hit system that processes projectile hits, melee hits, vore hits, etc.
/*
pub fn projectile_hit (
    mut ev_stat_change: EventWriter<StatChangeEvent>,
    mut ev_projectile_hit: EventReader<ProjectileHitEvent>,

    name_query: Query<&Name>,
    stat_query: Query<&Stats>,
) {

    for ev in ev_projectile_hit.iter() {

        let mut attacker_name = ev.targetting_entity.id().to_string();
        if let Ok(name) = name_query.get(ev.targetting_entity) {
            attacker_name = name.to_string();
        }

        let mut attacked_name = ev.hit_entity.id().to_string();
        if let Ok(name) = name_query.get(ev.hit_entity) {
            attacked_name = name.to_string();
        }

        let vars = HashMap::from([
            ("attacker".to_string(), attacker_name),
            ("attacked".to_string(), attacked_name),
            ("amount".to_string(), attack.damage.total.to_string()),
        ]);
    }

    // TODO: We need to get this info when we hit an entity.
    //let mut attacked_name = ev.target_entity.id().to_string();
    //if let Ok(name) = name_query.get(ev.target_entity) {
    //    attacked_name = name.to_string();
    //}

    // Do this if our projectile hits.
    /*
    ev_stat_change.send(StatChangeEvent{stat: attack.damage_type, amount: attack.damage.total, entity: ev.bumped_entity});



    let text_index = rng.gen_range(0..attack.interact_text.len());
    log.log_string_formatted(format![" {}", strfmt(&attack.interact_text[text_index], &vars).unwrap()], Color::RED);
    // 0.2.0 TODO: Add verbose dice rolls resource (bool)
    //             Log verbose dice rolls if resource is true
    */

    // TODO: Log something special if we miss our target or if we hit something besides our intended target.
}
 */

// Helper functions
fn rotate_point(pivot: Vec2, point: Vec2, rotation: f32) -> Vec2 {
    let sin = rotation.sin();
    let cos = rotation.cos();

    Vec2::new(cos * (point.x - pivot.x) - sin * (point.y - pivot.y) + point.x,
              sin * (point.x - pivot.x) + cos * (point.y - pivot.y) + point.y)
}

fn get_line_points(point_a: Vec2, point_b: Vec2, distance: f32) -> VecDeque<IVec2> {
    let mut points = VecDeque::new();
    //let distance = point_a.distance(point_b);
    for step in 0..=distance as i32 {
        let s = if distance == 0.0 {0.0} else {step as f32 / distance};

        let point = point_a.lerp(point_b, s).round().as_ivec2();

        if let Some(last_point) = points.back() {
            if last_point != &point {
                points.push_back(point);
            }
        }
        else {
            points.push_back(point);
        }
    }

    points
}

// Data
/// Should be fine? TODO: Remove uncertainty if this works fine.  
/// We can make count into a Dice later if we need to. (Variable amount of projectiles per ranged attack)  
/// Blast tells us if all of the shots come out at once or as separate shots. Basically shotgun vs smg.  
///     If we blast, all the bullets will be spread in the same direction by the penalty, and then each individual pellet will be modified by the optimal spread.  
///     If we burst, all bullets will be randomly spread out by optimal spread minus.
/// Projectile has a cost independent from its attack. This seems weird but might work out fine and good? Will probably make more sense when/if projectiles and attacks and such are made into more abstract constructs that are chained together.
#[derive(Clone)]
pub struct Projectile {
    pub attack: Attack,
    pub count: u32,
    pub blast: bool,

    pub cost: Dice,
    pub cost_type: StatType,

    pub spread_save: i32,
    pub spread_save_type: StatType,
    pub optimal_spread: f32,
    pub spread_penalty: f32,

    pub range_save: i32,
    pub range_save_type: StatType,
    pub optimal_range: f32,
    pub range_penalty: f32,         

    // TODO: Add field for projectile tile
}
impl Default for Projectile {
    fn default() -> Self {
        Projectile {
            attack: Attack::default(),
            count: 3,
            blast: false,

            cost: Dice::new("0"),
            cost_type: StatType::Health,

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
pub struct RangedAttacker {
    pub projectiles: Vec<Projectile>,
}