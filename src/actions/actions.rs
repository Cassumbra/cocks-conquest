use bevy::{prelude::*, ecs::{system::SystemState, schedule::IntoSystemDescriptor}};
use dyn_clonable::clonable;
use multimap::MultiMap;
use thunderdome::{Arena, Index};

use crate::turn::Turns;

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
         .add_event::<ActionEvent>()
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
    world: &mut World,
) {
    let mut system_state: SystemState<(
        // TODO: Uh oh! Can't read and write at the same time!
        //EventWriter<ActionEvent>,
        EventReader<ActionEvent>,
        EventReader<ReceivedCharacter>,

        ResMut<Turns>,
    
        Query<(&Actions)>,
    )> = SystemState::new(world);

    let (
        //mut ev_actions_write,
        mut ev_actions_read,
        mut ev_char,
        
        mut turns,
        
        mut query
    ) = system_state.get_mut(world);

    let mut action_events = ev_actions_read.iter().map(|x| x.clone()).collect::<Vec<ActionEvent>>();

    if let Some(turn_ent) = turns.turn_ent() {
        if let Ok(actions) = query.get(turn_ent) {
            for ev in ev_char.iter() {
                let opt_action_index = actions.bindings.get(&Trigger::CharBinding(ev.char));
                if let Some(action_index) = opt_action_index {
                    let action = actions.actions[*action_index].clone();
                    action_events.push(ActionEvent{action, actor: ActorType::Entity(turn_ent), target: TargetType::None});
                }
                
            
                

                //ev_actions_write.send(ActionEvent{action, actor: ActorType::Entity(turn_ent), target: TargetType::None})
            }
        }
    }
    


    

    for ev in action_events.iter() {
        ev.action.do_action(world, &ev.actor, &ev.target);
    }
}

// Data
// Effects
#[clonable]
pub trait ActionEffect: Send + Sync + Clone {
    fn apply_effect(&self, world: &World, actor: &ActorType, target: &TargetType);
}

#[derive(Clone)]
pub struct StatChangeEffect {

}
impl ActionEffect for StatChangeEffect {
    fn apply_effect(&self, mut world: &World, actor: &ActorType, target: &TargetType) {
        todo!()
    }
}

#[derive(Clone)]
pub struct ConsolePrintEffect {
    pub print_string: String,
}
impl ActionEffect for ConsolePrintEffect {
    fn apply_effect(&self, mut _world: &World, _actor: &ActorType, _target: &TargetType) {
        println!("{}", self.print_string);
    }
}

// Conditions
#[clonable]
pub trait ActionCondition: Send + Sync + Clone {
    fn check_condition(&self, world: &World, actor: &ActorType, target: &TargetType) -> bool;
}

#[derive(Clone)]
pub struct ANDCondition {

}
impl ActionCondition for ANDCondition {
    fn check_condition(&self, world: &World, actor: &ActorType, target: &TargetType) -> bool {
        todo!()
    }
}

#[derive(Clone)]
pub struct ORCondition {

}
impl ActionCondition for ORCondition {
    fn check_condition(&self, world: &World, actor: &ActorType, target: &TargetType) -> bool {
        todo!()
    }
}

#[derive(Clone)]
pub struct NOTCondition {

}
impl ActionCondition for NOTCondition {
    fn check_condition(&self, world: &World, actor: &ActorType, target: &TargetType) -> bool {
        todo!()
    }
}

#[derive(Clone)]
pub struct IsTurnCondition {
    
}
impl ActionCondition for IsTurnCondition {
    fn check_condition(&self, world: &World, actor: &ActorType, target: &TargetType) -> bool {
        todo!()
    }
}

#[derive(Clone)]
pub enum ActorType {
    None, // How can you have an action that is triggered by nothing?
    //Tile(IVec2), // Maybe if we make tiles be distinct from entities.
    //System(dyn IntoSystemDescriptor<Params>), // Disgusting.
    Entity(Entity),
    //MultiActor(Entity), // Ehh?? Maybe??
}

#[derive(Clone)]
pub enum TargetType {
    None,
    Tile(IVec2),
    MultiTile(IVec2),
    Actor(Entity),
    MultiActor(Entity),
}

#[derive(Clone)]
pub struct Action {
    // AND by default
    pub conditions: Vec<Box<dyn ActionCondition>>,
    pub effects: Vec<Box<dyn ActionEffect>>,
    pub duration: Dice,
}
impl Action {
    pub fn do_action(&self, mut world: &World, actor: &ActorType, target: &TargetType) -> bool {
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

#[derive(Hash, PartialEq, Eq)]
pub enum Trigger {
    CharBinding(char),
    KeyCode(KeyCode),
    Bump,
    // AI triggers
    Melee,
    Ranged,
    Movement,
}

// Components
/// Actions only trigger on an entity's turn.
/// We might make another component for reaction based actions later if necessary, but hopefully it won't be.
#[derive(Component)]
pub struct Actions {
    pub actions: Arena<Action>,
    pub bindings: MultiMap<Trigger, Index>,
}

// Events
#[derive(Clone)]
pub struct ActionEvent {
    pub action: Action,
    pub actor: ActorType,
    pub target: TargetType,
}