use bevy::prelude::*;

use crate::{turn::Turns, data::Position, actors::{TakesTurns}, actions::{movement::PointMoveEvent, interactions::MeleeAttacker}};

use super::targetting_behavior::Target;




pub fn melee_behavior (
    mut turns: ResMut<Turns>,
    
    mut ev_movement_event: EventWriter<PointMoveEvent>,

    mut ai_query: Query<(&Position, &Target, &MeleeAttacker), With<TakesTurns>>,
    actor_query: Query<(&Position), With<TakesTurns>>,
) {

}