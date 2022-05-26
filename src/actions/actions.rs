use bevy::prelude::*;

pub mod movement;
pub mod interactions;
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
         .add_event::<interactions::BumpEvent>()
         .add_event::<interactions::ActorRemovedEvent>()
         .add_event::<ranged::RandRangedAttackEvent>()
         .add_event::<healing::HealActionEvent>()
         .add_event::<movement::CollidableChangeEvent>()
         .add_event::<movement::PointMoveEvent>();

    }
}