use bevy::prelude::*;

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

// Data
pub trait ActionEffect {
    fn apply_effect(&self, mut world: &World, actor: Entity, target: TargetType) {}
}

pub trait ActionCondition {

}

pub struct StatChangeEffect {

}
impl ActionEffect for StatChangeEffect {
    
}

pub enum TargetType {
    Tile(IVec2),
    MultiTile(IVec2),
    Actor(Entity),
    MultiActor(Entity),
}

pub struct Action {
    pub conditions: Vec<Box<dyn ActionCondition>>,
    pub effects: Vec<Box<dyn ActionEffect>>,
    pub duration: Dice,
}
impl Action {
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

// Events
pub struct ActionEvent {
    pub action: Action,
    pub actor: Entity,
    pub target: TargetType,
}