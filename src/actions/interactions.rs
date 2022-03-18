// Unglob later
use bevy::prelude::*;

use super::super::{Position, Entity, Turns, TakesTurns};
use super::super::actors::player::Player;
use super::movement::Collidables;

// Events
pub struct BumpEvent {
    pub bumping_entity: Entity,
    pub bumped_entity: Entity,
}

// Systems
pub fn melee_attack (
    mut commands: Commands,
    attacker_query: Query<(Entity, &Position), With<TakesTurns>>,
    player_query: Query<&Position, With<Player>>,
    mut turns: ResMut<Turns>,
    collidables: Res<Collidables>,
) {

}