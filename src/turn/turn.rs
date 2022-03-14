// Unglob later
use bevy::prelude::*;
use super::*;

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
    mut commands: Commands,
    mut turns: ResMut<Turns>,
) {
    if turns.progress {

        let mut next_turn = turns.current + 1;
        if next_turn > turns.order.len() - 1 {
            next_turn = 0;
        }

        commands.entity(turns.order[turns.current]).remove::<IsTurn>();
        commands.entity(turns.order[next_turn]).insert(IsTurn);

        turns.current = next_turn;
        turns.progress = false;
    }
}

pub fn ensure_turn_exists(
    query: Query<Entity, With<IsTurn>>,
    mut commands: Commands,
    mut turns: ResMut<Turns>,
) {
    if !query.iter().next().is_some() && turns.order.len() != 0 {
        commands.entity(turns.order[turns.current]).insert(IsTurn);
    }
}