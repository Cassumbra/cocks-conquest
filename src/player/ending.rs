use bevy::prelude::*;

use crate::{actors::{TakesTurns, ActorRemovedEvent}, log::Log, turn::{Turns, TurnEvent}};

use super::Player;



pub fn player_victory (
    enemy_query: Query<&TakesTurns, Without<Player>>,

    mut ev_actor_remove: EventReader<ActorRemovedEvent>,

    mut log: ResMut<Log>,
) {
    for ev in ev_actor_remove.iter() {
        if enemy_query.is_empty() {
            log.log_string_formatted(" You win! Press shift+r to start over.".to_string(), Color::YELLOW)
        }
    }
}

pub fn player_death (
    player_query: Query<&Player>,

    mut ev_actor_remove: EventReader<ActorRemovedEvent>,

    mut log: ResMut<Log>,
    mut turns: ResMut<Turns>,
) {
    for ev in ev_actor_remove.iter() {
        if ev.frame_valid() && player_query.get(ev.removed_actor).is_ok() {
            log.log_string_formatted(" You have died! Press shift+r to try again.".to_string(), Color::YELLOW);
        }
    }
}