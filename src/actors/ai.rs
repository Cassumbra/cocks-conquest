use std::collections::VecDeque;
use bevy::utils::HashMap;
use sark_grids::Grid;
use rand::Rng;
use crate::actions::movement::{PointMoveEvent, Collidables};

use super::*;
use super::player::Player;

// Components
#[derive(Copy, Clone, PartialEq)]
pub enum AIState{Wander, EngageMelee, EngageRanged}
impl Default for AIState {
    fn default() -> AIState {
        AIState::Wander
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum AITarget{Entity(Entity), Location(Rectangle), None}
impl Default for AITarget {
    fn default() -> AITarget {
        AITarget::None
    }
}

#[derive(Component, Clone, PartialEq)]
pub struct AI{
    pub ai_state: AIState,
    pub target: AITarget,
    pub path: Vec<IVec2>,
    pub movements_allowed: Vec<IVec2>,
}
impl Default for AI {
    fn default() -> AI {
        AI {
            ai_state: AIState::Wander,
            target: AITarget::None,
            path: Vec::<IVec2>::new(),
            movements_allowed: vec![
                // Cardinals first. We should only look at diagonals if they're faster.
                // We're going around the way we would go around a circle in radians too. Idk why, just feels right.
                IVec2::new(1, 0), IVec2::new(0, 1), IVec2::new(-1, 0), IVec2::new(0, -1),
                IVec2::new(1, 1), IVec2::new(-1, 1), IVec2::new(-1, -1), IVec2::new(1, -1),
            ]
        }
    }
}
impl AI {
    pub fn bfs (&self, start: IVec2, goal: IVec2, collidables: &Grid<Option<Entity>>) -> Vec<IVec2> {
        // Set our start to the frontier zone, with no origin point.
        let mut frontier = VecDeque::<IVec2>::new();
        frontier.push_back(start);

        let mut came_from: HashMap<IVec2, Option<IVec2>> = HashMap::default();
        came_from.insert(start, None);

        let mut true_collidables = collidables.clone();
        true_collidables[start] = None;
        true_collidables[goal] = None;
        
        'full: while !frontier.is_empty() {
            let current = frontier.pop_front().expect("Frontier unexpectedly empty!");
            for direction in self.movements_allowed.iter() {
                let next = current + *direction;
                if !true_collidables[next].is_some() && !came_from.contains_key(&next) {
                    frontier.push_back(next);
                    came_from.insert(next, Some(current));
                }
                if next == goal {
                    break 'full;
                }
            }
        }
        
        

        let mut current = goal;
        let mut path = VecDeque::<IVec2>::new();

        while current != start {
            path.push_front(current);
            if came_from.contains_key(&current) {
                current = came_from[&current].expect("Path unexpectedly empty!");
            } else {
                println!("No path found!");
                break
            }
        }

        path.push_front(start);
        


        Vec::<IVec2>::from(path)
    }
}

// Systems
// We should probably make this pathfind to any entity OK, even if it isnt the player. Soon.
pub fn generic_brain (
    mut ev_movement_event: EventWriter<PointMoveEvent>,
    mut query: Query<(Entity, &Position, &mut AI)>,
    player_query: Query<&Position, With<Player>>,
    mut turns: ResMut<Turns>,
    collidables: Res<Collidables>,
    rooms: Res<Rooms>,
) {
    let mut collidable_map = collidables.0.clone();
    for (_other_ents, other_pos, _other_ai) in query.iter() {
        collidable_map[other_pos.0] = None;
    }


    let entity = turns.order[turns.current];
    if let Ok((ent, pos, mut ai)) = query.get_mut(entity) {
        let mut rng = rand::thread_rng();

        match ai.ai_state {
            AIState::EngageMelee | AIState::EngageRanged => {
                // Create path to player
                // Save path. Modify path if needed instead of doing complete recalculations
                if let Some(player_pos) = player_query.iter().next() {
                    ai.path = ai.bfs(pos.0, player_pos.0, &collidable_map);
                } else {
                    //println!("No player to gank!");
                    ai.ai_state = AIState::Wander;
                }
                
                if ai.ai_state == AIState::EngageMelee {
                    // Path to location adjacent to player. Whack player if adjacent.
                    let to_move = ai.path[1];
                    let delta = to_move - pos.0;
                    ev_movement_event.send(PointMoveEvent{
                        entity: ent,
                        movement: delta,
                    });
                }
                else {
                    // Path to location a few tiles away from player within line of sight of player. Shoot at player if in position.
                }
            }
            AIState::Wander => {
                if !matches!(ai.target, AITarget::Location(..)) {

                    let mut room_index = rng.gen_range(0..rooms.0.len());
                    let mut room = rooms.0[room_index];
                    
                    while room.intersect(&Rectangle::new(pos.0, 0, 0)) {
                        room_index = rng.gen_range(0..rooms.0.len());
                        room = rooms.0[room_index];
                    }
                    ai.target = AITarget::Location(room);
                }
            }
        }
    
        match ai.target {
            AITarget::Location(target) => {
                if target.intersect(&Rectangle::new(pos.0, 0, 0)) {
                    println!("Dmanb i got here");
                    let mut room_index = rng.gen_range(0..rooms.0.len());
                    let mut room = rooms.0[room_index];
                    while room.intersect(&Rectangle::new(pos.0, 0, 0)) {
                        println!("Fuck I'm already here");
                        room_index = rng.gen_range(0..rooms.0.len());
                        room = rooms.0[room_index];
                        
                    }
                    ai.target = AITarget::Location(room);
                }
                

                ai.path = ai.bfs(pos.0, target.center(), &collidables.0);

                if ai.path.len() != 1 {
                    let to_move = ai.path[1];
                    let delta = to_move - pos.0;
                    ev_movement_event.send(PointMoveEvent{
                        entity: ent,
                        movement: delta,
                    });
                } else {
                    println!("Fuuuuccck!!");
                }
            }
            
            _ => {

            }
        }

        turns.progress_turn();
    }
}