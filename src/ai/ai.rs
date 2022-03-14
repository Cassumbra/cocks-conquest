use std::cmp::max;

// Unglob later
use bevy::prelude::*;
use bevy::core::*;
use rand::Rng;
use pathfinding::prelude::{astar, absdiff};
use sark_grids::Grid;
use super::*;


pub fn generic_brain (
    mut ev_movement_event: EventWriter<PointMoveEvent>,
    mut query: Query<(Entity, &Position, &mut AI), (With<TakesTurns>)>,
    player_query: Query<&Position, (With<Player>)>,
    mut turns: ResMut<Turns>,
    collidables: Res<Collidables>,
) {
    if let Some(player_pos) = player_query.iter().next() {
        let mut collidable_map = collidables.0.clone();
        collidable_map[player_pos.0] = None;

        for (ent, pos, mut ai) in query.iter_mut() {
            collidable_map[pos.0] = None;

            if turns.is_turn(&ent) {
                match ai.ai_state {
                    AIState::EngageMelee | AIState::EngageRanged => {
                        // Create path to player
                        // Save path. Modify path if needed instead of doing complete recalculations
                        ai.path = ai.bfs(pos.0, player_pos.0, &collidable_map);

                        

                        if ai.ai_state == AIState::EngageMelee {
                            // Path to location adjacent to player. Whack player if adjacent.

                            

                            let to_move = ai.path[1];
                            let delta = to_move - pos.0;
                            ev_movement_event.send(PointMoveEvent{
                                entity: ent,
                                movement: delta,
                            });

                            println!("{:?}", delta);
                        }
                        else {
                            // Path to location a few tiles away from player within line of sight of player. Shoot at player if in position.
                        }
                    }
                    AIState::Wander => {
                        // Path to random open location.
                        // Save path and only recalculate if obstructed or if has reached goal.
                    }
                }
            
                turns.progress_turn();
            }
        }
    }
}

pub fn walk_at_player (
    mut ev_movement_event: EventWriter<PointMoveEvent>,
    query: Query<(Entity, &Position), (With<TakesTurns>, With<AIWalkAtPlayer>)>,
    player_query: Query<&Position, (With<Player>)>,
    mut turns: ResMut<Turns>,
    collidables: Res<Collidables>,
) {
    if let Some(player_pos) = player_query.iter().next() {
        let mut ignore_positions = Vec::<IVec2>::new();

        for (ent, pos) in query.iter() {
            ignore_positions.push(pos.0);

            if turns.is_turn(&ent) {
                let collidable_map = collidables.0.clone();
                


                let path = astar(
                    &pos.0,
                    |p| check_movement(vec![*p + IVec2::new(-1, 1), *p + IVec2::new(0, 1), *p + IVec2::new(1, 1),
                                            *p + IVec2::new(-1, 0), *p + IVec2::new(1, 0),
                                            *p + IVec2::new(-1, -1), *p + IVec2::new(0, -1), *p + IVec2::new(0, 1)],
                                            &ignore_positions, player_pos.0, &collidable_map),
                    |p| 1000 * max(absdiff(p.x, player_pos.0.x), absdiff(p.y, player_pos.0.y))
                        + if ignore_positions.contains(&pos.0) {0} else {0},
                    |p| *p == player_pos.0
            
                );
                if let Some(path_ok) = path {
                    let to_move = path_ok.0[1];
                    let delta = to_move - pos.0;
                    ev_movement_event.send(PointMoveEvent{
                        entity: ent,
                        movement: delta,
                    });
                }
                else {
                    println!("{:?} waits.", ent);
                }

                turns.progress_turn();

                //(i32::pow(absdiff(p.x, player_pos.0.x), 2) + i32::pow(absdiff(p.y, player_pos.0.y), 2)) as f64).sqrt() as i32 
            }
        }
    }
} 

pub fn check_movement (
    positions: Vec<IVec2>,
    ignore_positions: &Vec<IVec2>,
    target_position: IVec2,
    collidables: &Grid<Option<Entity>>,

) -> Vec<(IVec2, i32)> {
    let mut available_moves = Vec::<(IVec2, i32)>::new();

    for position in positions.iter() {

        if *position == target_position {
            available_moves.push((*position, 1000));
        }
        else if ignore_positions.contains(position) {
            available_moves.push((*position, 128000));
        }
        else if !collidables[*position].is_some() {
            available_moves.push((*position, 1000));
        }
    }

    available_moves
}


pub fn do_nothing (
    query: Query<Entity, (With<TakesTurns>, With<AIDoNothing>)>,
    mut turns: ResMut<Turns>,
) {
    for ent in query.iter() {
        if turns.is_turn(&ent) {
            println!("{:?}", ent.id());
            turns.progress_turn();
        }
    }
}