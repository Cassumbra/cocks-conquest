use bevy::prelude::*;

use crate::{turn::Turns, data::Position, actors::{TakesTurns, Moves}, actions::{movement::PointMoveEvent, melee::MeleeAttacker}};

use super::targetting_behavior::Engages;


pub fn melee_behavior (
    mut turns: ResMut<Turns>,
    
    mut ev_movement_event: EventWriter<PointMoveEvent>,

    mut ai_query: Query<(&Position, &Engages, &MeleeAttacker, &Moves), With<TakesTurns>>,
    target_query: Query<(&Position)>,
) {
    // TODO: Maybe we should turn this into a system condition?
    if turns.progress == true {
        return;
    }

    let ai_ent = turns.order[turns.current];
    if let Ok((pos, engagement, attacker, moves)) = ai_query.get_mut(ai_ent) {

        if engagement.target.is_none() {
            return;
        }

        if let Ok(target_pos) = target_query.get(engagement.target.unwrap()) {
            // Check if any of our moves can take us to our target.
            for m in moves.iter() {
                if **pos + *m == **target_pos {
                    ev_movement_event.send(PointMoveEvent{entity: ai_ent, movement: *m});
                    turns.progress_turn();
                }
            }
        }
    }
}