use bevy::prelude::*;
use super::*;

#[derive(Default)]
pub struct TurnPlugin;

// 
pub struct StartTurnEvent {
    pub entity: Entity,
}

// Systems
/// Updates the order in which entities take turns.
/// Only gets updated when necessary.
pub fn update_turn_order(
    //mut commands: Commands,
    query: Query<Entity, With<TakesTurns>>,
    turns_changed: Query<(Entity, &TakesTurns), Or<(Changed<TakesTurns>, Added<TakesTurns>)>>,
    mut turns: ResMut<Turns>,
) {
    if turns_changed.iter().next().is_some() {
        turns.order = Vec::new();
        for ent in query.iter() {
            turns.order.push(ent);
        }
    }
}

pub fn update_turn(
    mut turns: ResMut<Turns>,
) {
    if turns.progress {

        let mut next_turn = turns.current + 1;
        if next_turn > turns.order.len() - 1 {
            next_turn = 0;
        }

        turns.current = next_turn;
        turns.progress = false;
    }
}