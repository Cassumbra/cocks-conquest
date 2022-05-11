use std::cmp::{Ordering, Reverse};
use std::collections::{VecDeque, BinaryHeap};
use bevy::utils::{HashMap, HashSet};
use sark_grids::Grid;
use rand::Rng;
use crate::actions::interactions::TargetEvent;
use crate::actions::movement::{PointMoveEvent, Collidables};

use super::*;
use super::player::Player;
use super::status_effects::Tranced;

// Data
#[derive(Copy, Clone, PartialEq)]
struct Engagement {
    distance: f32,
    entity: Entity,
}

#[derive(Copy, Clone, PartialEq)]
pub enum AIState{Wander(Rectangle), Engage(Engagement), None}
impl Default for AIState {
    fn default() -> AIState {
        AIState::None
    }
}

// Position sortable by weight.
#[derive(Copy, Clone, PartialEq, Eq)]
struct WeightedPosition {
    position: IVec2,
    weight: u32,
}
impl Ord for WeightedPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}
impl PartialOrd for WeightedPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.weight.cmp(&other.weight))
    }
}

// Components
#[derive(Component, Clone, PartialEq)]
pub struct AI{
    pub state: AIState,
    pub halted_count: u8,
    pub path: VecDeque<IVec2>,
    pub movements_allowed: Vec<IVec2>,
}
impl Default for AI {
    fn default() -> AI {
        AI {
            state: AIState::None,
            halted_count: 0,
            path: VecDeque::<IVec2>::new(),
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
    fn bfs (&self, start: IVec2, goal: IVec2, collidables: &Grid<Option<Entity>>) -> VecDeque<IVec2> {
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
                // Early exit
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
                println!("No path found from {:?} to {:?}.", start, goal);
                path.clear();
                break
            }
        }
        path.push_front(start);
        
        path
    }

    fn dijkstra (&self, start: IVec2, goal: IVec2, collidables: &Grid<Option<Entity>>, obstacles: &Grid<u32>) -> (VecDeque<IVec2>, u32) {
        //IVec2s do not support Ord. Sad!
        let mut frontier = BinaryHeap::<Reverse<WeightedPosition>>::new();
        frontier.push(Reverse(WeightedPosition{position: start, weight: 0}));

        let mut came_from: HashMap<IVec2, Option<IVec2>> = HashMap::default();
        came_from.insert(start, None);

        let mut cost_so_far: HashMap<IVec2, u32> = HashMap::default();
        cost_so_far.insert(start, 0);

        let mut true_collidables = collidables.clone();
        true_collidables[start] = None;
        true_collidables[goal] = None;
        
        'full: while !frontier.is_empty() {
            let current = frontier.pop().expect("Frontier unexpectedly empty!").0.position;

            for direction in self.movements_allowed.iter() {
                let next = current + *direction;
                let new_cost = cost_so_far[&current] + obstacles[next] + 1;
                if !true_collidables[next].is_some() && (!cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next]) {
                    cost_so_far.insert(next, new_cost);
                    let priority = new_cost;
                    frontier.push(Reverse(WeightedPosition{position: next, weight: priority}));
                    came_from.insert(next, Some(current));
                }
                // Early exit
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
                println!("No path found from {:?} to {:?}.", start, goal);
                path.clear();
                break
            }
        }
        path.push_front(start);
        
        if cost_so_far.contains_key(&goal) {
            (path, cost_so_far[&goal])
        } else {
            (path, u32::MAX)
        }
        
    }
}

// Systems
// We should probably make this pathfind to any entity OK, even if it isnt the player. Soon.
// We should probably split this up into three different systems later. Or more.
// State change handling, Pathing, Actions
pub fn generic_brain (
    mut ev_target_event: EventWriter<TargetEvent>,
    mut ev_movement_event: EventWriter<PointMoveEvent>,

    mut ai_query: Query<(&Position, &mut AI, &Vision), Without<Tranced>>,
    actors_query: Query<&Position, With<TakesTurns>>,
    player_query: Query<(Entity, &Position), With<Player>>,
    mut turns: ResMut<Turns>,
    collidables: Res<Collidables>,
    rooms: Res<Rooms>,
) {
    let ai_ent = turns.order[turns.current];
    if let Ok((ai_pos, mut ai, vis)) = ai_query.get_mut(ai_ent) {
        let mut true_collidables = collidables.0.clone();
        true_collidables[ai_pos.0] = None;

        let mut non_actor_collidables = collidables.0.clone();
        let mut obstacles = Grid::<u32>::new(0, [collidables.0.width(), collidables.0.height()]);
        for actor_pos in actors_query.iter() {
            non_actor_collidables[actor_pos.0] = None;
            obstacles[actor_pos.0] = 8;
        }

        

        let mut rng = rand::thread_rng();

        // Later we'll make this work for nonplayers that are aligned against this ai.
        if let Some((player_ent, player_pos)) = player_query.iter().next() {
            // Path to the player if we see them.
            if !matches!(ai.state, AIState::Engage(..)) && vis.0.visible[player_pos.0] {
                // TODO: Maybe this should be defined somewhere in the ai itself?
                ai.state = AIState::Engage(Engagement{distance: 3.5, entity: player_ent })
            }
            // Wander around if there's nothing better for us to do.
            else if matches!(ai.state, AIState::None) {
                let room_index = rng.gen_range(0..rooms.0.len());
                let mut room = rooms.0[room_index];

                while room.intersect(&Rectangle::new(ai_pos.0, 0, 0)) {
                    let room_index = rng.gen_range(0..rooms.0.len());
                    room = rooms.0[room_index];
                    ai.state = AIState::Wander(room);
                }

                ai.state = AIState::Wander(room);
            }
        }

        let mut need_to_recalculate = true;
        let mut need_to_move = false;

        match ai.state {
            AIState::Engage(engagement) => {
                if let Ok(target_pos) = actors_query.get(engagement.entity) {
                    true_collidables[target_pos.0] = None;
                    if ai.path.len() > 1 {
                        need_to_recalculate = ai.path.back().unwrap().as_vec2().distance(target_pos.0.as_vec2()) > engagement.distance;
                        need_to_move = ai_pos.0.as_vec2().distance(target_pos.0.as_vec2()) > engagement.distance;
                    }
                    
                    if ai.path.len() == 0 || ai.path.iter().any(|p| true_collidables[*p].is_some()) ||
                        need_to_recalculate
                    {
                        let mut adjacent_path = (VecDeque::<IVec2>::new(), u32::MAX);
                        let mut direct_path = (VecDeque::<IVec2>::new(), u32::MAX);

                        let mut positions = positions_dist_away(target_pos.0, engagement.distance);
                        positions.sort_unstable_by_key(|p| (p.as_vec2().distance(target_pos.0.as_vec2() * 1000.0) as i32));
                        for p in positions {
                            if !collidables.0[p].is_some() {
                                adjacent_path = ai.dijkstra(ai_pos.0, p, &non_actor_collidables, &obstacles);
                                break;
                            }
                        }

                        direct_path = ai.dijkstra(ai_pos.0, target_pos.0, &non_actor_collidables, &obstacles);

                        //if adjacent_path.1 < direct_path.1 {
                        //    println!("Adjacent path cheaper.");
                            ai.path = adjacent_path.0;
                        //} else {
                        //    println!("Direct path cheaper.");
                            ai.path = direct_path.0;
                        //}
                    }
                }
            }
            AIState::Wander(area) => {
                // if we don't have a path or our path is blocked:
                if ai.path.len() == 0 || ai.path.iter().any(|p| collidables.0[*p].is_some()) {
                    ai.path = ai.dijkstra(ai_pos.0, area.center(), &non_actor_collidables, &obstacles).0;
                }
                if area.intersect(&Rectangle::new(ai_pos.0, 0, 0)) {

                    ai.state = AIState::None;
                }

                need_to_move = true;
                
            }
            AIState::None => {
                ai.path = VecDeque::<IVec2>::new()
            }
        }
        
        if need_to_move {
            if ai.path.len() < 1 || (ai.path.len() > 0 && collidables.0[ai.path[1]].is_some()) {
                ai.halted_count += 1;
                if ai.halted_count == 3 {
                    ai.halted_count = 0;
                    ai.state = AIState::None;
                }
            } else {
                let to_move = ai.path[1];
                let delta = to_move - ai_pos.0;
                ev_movement_event.send(PointMoveEvent{
                    entity: ai_ent,
                    movement: delta,
                });
                ai.path.pop_front();
            }
        } 
        else {
            //ai.path = VecDeque::from([ai_pos.0, ai_pos.0]);
            match ai.state {
                AIState::Engage(engagement) => {
                    if let Ok(target_pos) = actors_query.get(engagement.entity) {
                        if ai_pos.0.as_vec2().distance(target_pos.0.as_vec2()) <= 1.5 {
                            let delta = target_pos.0 - ai_pos.0;
                            ev_movement_event.send(PointMoveEvent{
                                entity: ai_ent,
                                movement: delta,
                            });
                        } else if ai_pos.0.as_vec2().distance(target_pos.0.as_vec2()) <= engagement.distance {
                            println!("AI DO A SHOOTY");
                            ev_target_event.send(TargetEvent{
                                targetting_entity: ai_ent,
                                target: target_pos.0,
                            })
                        }
                    }
                }

                _ => {

                }
            }
        }

        
        turns.progress_turn();
    }
}

pub fn tranced_brain (
    ai_query: Query<&AI, With<Tranced>>,

    mut turns: ResMut<Turns>,
) {
    let ai_ent = turns.order[turns.current];
    if let Ok(_ai) = ai_query.get(ai_ent) {
        turns.progress_turn();
    }
}

// Algorithm retrieved from: https://www.redblobgames.com/grids/circle-drawing/ Section 4: Outlines
fn positions_dist_away(
    position: IVec2,
    distance: f32,
) -> Vec<IVec2> {
    let mut positions = Vec::<IVec2>::new();
    for r in 0..=(distance * (0.5_f32).sqrt()).floor() as i32 {
        let d = (distance*distance - (r*r) as f32).sqrt() as i32;
        positions.push(IVec2::new(position.x - d, position.y + r));
        positions.push(IVec2::new(position.x + d, position.y + r));
        positions.push(IVec2::new(position.x - d, position.y - r));
        positions.push(IVec2::new(position.x + d, position.y - r));

        positions.push(IVec2::new(position.x - r, position.y + d));
        positions.push(IVec2::new(position.x + r, position.y + d));
        positions.push(IVec2::new(position.x - r, position.y - d));
        positions.push(IVec2::new(position.x + r, position.y - d));
    }

    let mut uniques = HashSet::<IVec2>::default();
    positions.retain(|e| uniques.insert(*e));

    positions
}

// Kinda cool but mostly useless.
// We also need to check if the ai's goal corresponds to a position around the player
// Actually, maybe it wouldn't be so bad if we simply checked a rect around the player?
// We're going to use a modified version of this.
// if ai.path.back() ==  || ai.path.len() == 0 || ai.path.iter().any(|p| collidables.0[*p].is_some()) {

/*
Create a target to pathfind towards if we have none.
If we don't have a valid path, generate a new one after a few turns.
Move one step along our path.
If we haven't moved already, perform some action (if applicable)
*/