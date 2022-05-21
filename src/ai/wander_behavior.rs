use std::collections::VecDeque;

use bevy::prelude::*;
use rand::Rng;
use sark_grids::Grid;

use crate::actions::movement::{Collidables, PointMoveEvent};
use crate::actors::{Moves, TakesTurns};
use crate::data::{Rectangle, Position};
use crate::turn::Turns;

use super::{dijkstra, Path};


pub fn wander_behavior (
    mut turns: ResMut<Turns>,
    collidables: Res<Collidables>,
    
    mut ev_movement_event: EventWriter<PointMoveEvent>,

    mut ai_query: Query<(&Position, &mut Wanderer, &Moves), With<TakesTurns>>,
    actor_query: Query<(&Position), With<TakesTurns>>,
) {
    // TODO: Maybe we should turn this into a system condition?
    if turns.progress == true {
        println!("im outta here!");
        return;
    }

    let ai_ent = turns.order[turns.current];
    if let Ok((pos, mut wanderer, moves)) = ai_query.get_mut(ai_ent) {
        let mut rng = rand::thread_rng();

        let mut path_generation_attempts = 3;
    
        while wanderer.path.positions.is_empty() || collidables.0[wanderer.path.positions[0]].is_some() {
            let mut non_actor_collidables = collidables.0.clone();
            let mut obstacles = Grid::<u32>::new(0, [collidables.0.width(), collidables.0.height()]);
            for actor_pos in actor_query.iter() {
                non_actor_collidables[actor_pos.0] = None;
                obstacles[actor_pos.0] = 8;
            }

            wanderer.target_index = rng.gen_range(0..wanderer.locations.len());

            wanderer.path = dijkstra(&**pos, &wanderer.get_target().center(), &**moves, &non_actor_collidables, &obstacles);

            path_generation_attempts -= 1;
            if path_generation_attempts == 0 {
                break;
            }
        }

        //println!("destination: {:?}", wanderer.path.positions[0]);
        if !collidables.0[wanderer.path.positions[0]].is_some() {
            let to_move = wanderer.path.positions[0];
            let delta = to_move - **pos;
            ev_movement_event.send(PointMoveEvent{
                entity: ai_ent,
                movement: delta,
            });
            wanderer.path.positions.pop_front();
        }

        turns.progress_turn();
        //println!("{}", turns.progress);
    }
}

// Components
#[derive(Component)]
pub struct Wanderer {
    pub locations: Vec<Rectangle>,
    pub target_index: usize,
    pub path: Path,
}
impl Wanderer {
    pub fn new(locations: Vec<Rectangle>) -> Wanderer {
        Wanderer {locations, target_index: 0, path: Path {positions: VecDeque::new(), cost: 0}}
    }

    pub fn get_target(&self) -> Rectangle {
        self.locations[self.target_index]
    }
}