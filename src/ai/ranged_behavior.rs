use bevy::prelude::*;

use crate::{turn::Turns, data::Position, actors::{TakesTurns, Moves, vision::Vision}, actions::ranged::{RandRangedAttackEvent, RangedAttacker}};

use super::targetting_behavior::Engages;


pub fn ranged_behavior (
    mut turns: ResMut<Turns>,
    
    mut ev_attack_event: EventWriter<RandRangedAttackEvent>,

    mut ai_query: Query<(&Position, &Engages, &RangedAttacker, &Moves, &Vision), With<TakesTurns>>,
    target_query: Query<(&Position)>,
) {
    // TODO: Maybe we should turn this into a system condition?
    if turns.progress == true {
        return;
    }

    let ai_ent = turns.order[turns.current];
    if let Ok((pos, engagement, attacker, moves, vision)) = ai_query.get_mut(ai_ent) {

        if engagement.target.is_none() {
            return;
        }



        if let Ok(target_pos) = target_query.get(engagement.target.unwrap()) {

            // Check if target is visible
            if !vision.visible(**target_pos) {
                return;
            }

            ev_attack_event.send(RandRangedAttackEvent{targetting_entity: ai_ent, target: **target_pos});
            turns.progress_turn();
        }
    }
}