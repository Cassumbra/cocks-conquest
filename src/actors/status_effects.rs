use bevy::prelude::*;

use crate::turn::Turns;

use super::TakesTurns;





// Systems
// I don't know if I want to move this to be with the rest of the behaviors or not.
// It may not only affect AI.
pub fn tranced_behavior (
    mut turns: ResMut<Turns>,

    tranced_query: Query<&Tranced, With<TakesTurns>>,
) {
    // TODO: Maybe we should turn this into a system condition?
    if turns.progress == true {
        println!("no trancing for me!");
        return;
    }

    let entity = turns.order[turns.current];
    if let Ok(_tranced) = tranced_query.get(entity) {
        println!("wuhh im tranced woah");
        turns.progress_turn();
    }
}

// Components
#[derive(Component, Clone)]
pub struct Tranced;

#[derive(Component, Clone, Deref, DerefMut)]
pub struct StatusEffects (Vec<StatusEffect>);

// Data
#[derive(Clone)]
pub enum StatusEffectType {
    Tranced,
    Sneaking,

}

#[derive(Clone)]
pub struct StatusEffect {
    pub status_type: StatusEffectType,
    pub duration: u32,
}