// Unglob later
use bevy::prelude::*;
use rand::Rng;
use pathfinding::prelude::{astar, absdiff};
use sark_grids::Grid;
use super::super::*;

pub fn melee_attack (
    mut commands: Commands,
    attacker_query: Query<(Entity, &Position), (With<TakesTurns>, With<AIWalkAtPlayer>, With<IsTurn>)>,
    player_query: Query<&Position, (With<Player>)>,
    mut turns: ResMut<Turns>,
    collidables: Res<Collidables>,
) {

}