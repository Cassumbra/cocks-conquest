use bevy::prelude::*;
use sark_grids::Grid;

use crate::{turn::Turns, data::Position, actors::{TakesTurns, Moves, vision::Vision}, actions::{movement::{PointMoveEvent, Collidables}}};

use super::{targetting_behavior::Engages, dijkstra};

pub fn engage_behavior (
    collidables: Res<Collidables>,
    mut turns: ResMut<Turns>,
    
    mut ev_movement_event: EventWriter<PointMoveEvent>,

    mut ai_query: Query<(&Position, &mut Engages, &Moves, &Vision), With<TakesTurns>>,
    target_query: Query<(&Position)>,
    actor_query: Query<(&Position), With<TakesTurns>>,
) {
    // TODO: Maybe we should turn this into a system condition?
    if turns.progress == true {
        return;
    }

    let ai_ent = turns.order[turns.current];
    if let Ok((pos, mut engagement, moves, vision)) = ai_query.get_mut(ai_ent) {

        if engagement.target.is_none() {
            return;
        }

        if let Ok(target_pos) = target_query.get(engagement.target.unwrap()) {
            let distance = target_pos.as_vec2().distance(pos.as_vec2());

            // Do nothing if we are already in range and can see our target.
            if distance <= engagement.distance && vision.visible(**target_pos) {
                return;
            }

            //if collidables.0[engagement.path.positions[0]].is_some() {
            //    engagement.path.positions.clear();
            //}

            // Reset pathing if it will not bring us closer to our target
            // TODO: This does not work properly currently. Fix this.
            //       It may technically be "working" but not how we want it to.
            /*
            if let Some(last_path_pos) = engagement.path.positions.back() {
                let path_distance = last_path_pos.as_vec2().distance(pos.as_vec2());
                if path_distance > engagement.distance {
                    engagement.path.positions.clear();
                }
            }
             */

            // TODO: This is temporary.
            engagement.path.positions.clear();

            if engagement.path.positions.is_empty() {
                let mut non_actor_collidables = collidables.0.clone();
                let mut obstacles = Grid::<u32>::new(0, [collidables.0.width(), collidables.0.height()]);
                for actor_pos in actor_query.iter() {
                    non_actor_collidables[actor_pos.0] = None;
                    obstacles[actor_pos.0] = 8;
                }
    
                engagement.path = dijkstra(&**pos, &target_pos, &**moves, &non_actor_collidables, &obstacles);
            }
    
            if collidables.0[engagement.path.positions[0]].is_none() {
                let to_move = engagement.path.positions[0];
                let delta = to_move - **pos;
                ev_movement_event.send(PointMoveEvent{
                    entity: ai_ent,
                    movement: delta,
                });
                engagement.path.positions.pop_front();
            }
    
            turns.progress_turn();
            //println!("{}", turns.progress);

        }
    }
}