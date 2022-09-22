use bevy::{prelude::*, utils::HashMap};
use multimap::MultiMap;
use thunderdome::{Arena, Index};

use self::attack::Dice;

pub mod movement;
pub mod attack;
pub mod healing;
pub mod vore;
pub mod melee;
pub mod ranged;


// Plugin
#[derive(Default)]
pub struct ActionPlugin;
impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app
         .add_event::<attack::BumpEvent>()
         .add_event::<attack::AttackEvent>()
         .add_event::<ranged::RandRangedAttackEvent>()
         .add_event::<ranged::RangedAttackEvent>()
         .add_event::<ranged::ProjectileHitEvent>()
         .add_event::<healing::HealActionEvent>()
         .add_event::<movement::CollidableChangeEvent>()
         .add_event::<movement::PointMoveEvent>();

    }
}

pub fn process_actions (
    mut ev_actions: EventReader<ActionEvent>,
    world: &mut World,
) {
    for ev in ev_actions.iter() {
        ev.action.do_action(world, &ev.actor, &ev.target);
    }
}

// Data
// Effects
pub trait ActionEffect: Send + Sync {
    fn apply_effect(&self, world: &World, actor: &Entity, target: &TargetType);
}

pub struct StatChangeEffect {

}
impl ActionEffect for StatChangeEffect {
    fn apply_effect(&self, mut world: &World, actor: &Entity, target: &TargetType) {
        todo!()
    }
}

pub struct ConsolePrintEffect {
    pub print_string: String,
}
impl ActionEffect for ConsolePrintEffect {
    fn apply_effect(&self, mut _world: &World, _actor: &Entity, _target: &TargetType) {
        println!("{}", self.print_string);
    }
}

// Conditions
pub trait ActionCondition: Send + Sync {
    fn check_condition(&self, world: &World, actor: &Entity, target: &TargetType) -> bool;
}

pub struct ANDCondition {

}
impl ActionCondition for ANDCondition {
    fn check_condition(&self, world: &World, actor: &Entity, target: &TargetType) -> bool {
        todo!()
    }
}

pub struct ORCondition {

}
impl ActionCondition for ORCondition {
    fn check_condition(&self, world: &World, actor: &Entity, target: &TargetType) -> bool {
        todo!()
    }
}

pub struct NOTCondition {

}
impl ActionCondition for NOTCondition {
    fn check_condition(&self, world: &World, actor: &Entity, target: &TargetType) -> bool {
        todo!()
    }
}

pub struct IsTurnCondition {
    
}
impl ActionCondition for IsTurnCondition {
    fn check_condition(&self, world: &World, actor: &Entity, target: &TargetType) -> bool {
        todo!()
    }
}


pub enum TargetType {
    Tile(IVec2),
    MultiTile(IVec2),
    Actor(Entity),
    MultiActor(Entity),
}

pub struct Action {
    // AND by default
    pub conditions: Vec<Box<dyn ActionCondition>>,
    pub effects: Vec<Box<dyn ActionEffect>>,
    pub duration: Dice,
}
impl Action {
    pub fn do_action(&self, mut world: &World, actor: &Entity, target: &TargetType) -> bool {
        for condition in &self.conditions {
            if !condition.check_condition(world, actor, target) {
                return false;
            }
        }
        
        for effect in &self.effects {
            effect.apply_effect(world, actor, target);
        }

        true
    }

    pub fn new_melee_attack() -> Action {
        todo!()
    }

    pub fn new_projectile() -> Action {
        todo!()
    }

    pub fn add_cost(&self) -> Action {
        todo!()
    }
}

pub enum Trigger {
    CharBinding(char)
}

pub struct ID {

}

// Components
/*
pub struct Actions<'a> {
    pub actions: Vec<Action>,
    pub bindings: MultiMap<Binding, &'a Action>, 
}
 */

pub struct Actions {
    pub actions: Arena<Action>,
    pub bindings: MultiMap<Trigger, Index>,
}

// Events
pub struct ActionEvent {
    pub action: Action,
    pub actor: Entity,
    pub target: TargetType,
}