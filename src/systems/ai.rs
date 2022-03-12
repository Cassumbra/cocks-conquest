// Unglob later
use bevy::prelude::*;
use rand::Rng;
use pathfinding::prelude::{astar, absdiff};
use sark_grids::Grid;
use super::super::*;

pub fn generic_brain (
    query: Query<(Entity, &Position), (With<TakesTurns>, With<AIState>)>,
    player_query: Query<&Position, (With<Player>)>,
    mut turns: ResMut<Turns>,
    collidables: Res<Collidables>,
) {
    for (ent, pos) in query.iter() {
        if turns.is_turn(&ent) {
            
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
        for (ent, pos) in query.iter() {
            if turns.is_turn(&ent) {
                let collidable_map = collidables.0.clone();
                //println!("collidables.0 is: {:?}", collidables.0);

                let path = astar(
                    &(pos.0.x, pos.0.y),
                    |&(x, y)| vec![(x-1, y+1), (x, y+1), (x+1, y+1),
                                    (x-1, y), (x+1, y),
                                    (x-1, y-1), (x, y-1), (x+1, y+1)]
                                .into_iter().map(|p| (p, 1)),
                    |&(x, y)| (absdiff(x, player_pos.0.x) + absdiff(y, player_pos.0.y)) * if collidables.0[[x as u32, y as u32 ]].is_some() {999} else {1},
                    |&p| p == (player_pos.0.x, player_pos.0.y)
            
                );
                //println!("{:?}", path);
                if let Some(path_ok) = path {
                    //println!("All good.");
                    let x_to_move = path_ok.0[1].0;
                    let y_to_move = path_ok.0[1].1;
                    let dx = x_to_move - pos.0.x;
                    let dy = y_to_move - pos.0.y;
                    //commands.entity(ent).insert(Movement(IVec2::new(dx, dy)));
                    ev_movement_event.send(PointMoveEvent{
                        entity: ent,
                        movement: IVec2::new(dx, dy),
                    });
                }
                else {
                    println!("{:?} waits.", ent);
                }

                turns.progress_turn();
            }
        }
    }
} 

pub fn can_move (
    x: i32,
    y: i32,
    collidables: &Grid<Option<Entity>>,
) -> bool {
    //println!("{}, {}", x, y);
    //!collidables[[x as u32, y as u32 ]].is_some()
    true
}

pub fn movement_options (
    x: i32,
    y: i32,
    collidables: &Grid<Option<Entity>>,
) -> Vec<(i32, i32)> {
    let mut available_moves = Vec::<(i32, i32)>::new();

    if can_move (x-1, y+1, collidables) {
        available_moves.push((x-1, y+1));
    }
    if can_move (x, y+1, collidables) {
        available_moves.push((x, y+1));
    }
    if can_move (x+1, y+1, collidables) {
        available_moves.push((x+1, y+1));
    }

    if can_move (x-1, y, collidables) {
        available_moves.push((x-1, y));
    }
    if can_move (x+1, y, collidables) {
        available_moves.push((x+1, y));
    }

    if can_move (x-1, y-1, collidables) {
        available_moves.push((x-1, y-1));
    }
    if can_move (x, y-1, collidables) {
        available_moves.push((x, y-1));
    }
    if can_move (x+1, y+1, collidables) {
        available_moves.push((x+1, y+1));
    }


    let cool_vec =  vec![(x-1, y+1), (x, y+1), (x+1, y+1),
                               (x-1, y), (x+1, y),
                               (x-1, y-1), (x, y-1), (x+1, y+1)];

    //println!("{:?}", available_moves);
    available_moves
    //cool_vec
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