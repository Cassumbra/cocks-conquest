use std::cmp::{Ordering, Reverse};
use std::collections::{VecDeque, BinaryHeap};
use bevy::utils::{HashMap, HashSet};
use sark_grids::Grid;
use rand::Rng;
use crate::actions::interactions::RandRangedAttackEvent;
use crate::actions::movement::{PointMoveEvent, Collidables};
use crate::actors::vision::Vision;

use super::*;
use super::player::Player;
use super::status_effects::Tranced;

pub mod engage_behavior;
pub mod wander_behavior;
pub mod targetting_behavior;
pub mod melee_behavior;
pub mod ranged_behavior;

// Data
#[derive(Copy, Clone, PartialEq)]
struct Engagement {
    distance: f32,
    entity: Entity,
}

#[derive(Clone, Default)]
pub struct Path {
    pub positions: VecDeque<IVec2>,
    pub cost: u32,
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

// Helper functions
fn dijkstra (start: &IVec2, goal: &IVec2, allowed_movements: &Vec<IVec2>, collidables: &Grid<Option<Entity>>, obstacles: &Grid<u32>) -> Path {
    //IVec2s do not support Ord. Sad!
    let mut frontier = BinaryHeap::<Reverse<WeightedPosition>>::new();
    frontier.push(Reverse(WeightedPosition{position: *start, weight: 0}));

    let mut came_from: HashMap<IVec2, Option<IVec2>> = HashMap::default();
    came_from.insert(*start, None);

    let mut cost_so_far: HashMap<IVec2, u32> = HashMap::default();
    cost_so_far.insert(*start, 0);

    let mut true_collidables = collidables.clone();
    true_collidables[*start] = None;
    true_collidables[*goal] = None;
    
    'full: while !frontier.is_empty() {
        let current = frontier.pop().expect("Frontier unexpectedly empty!").0.position;

        for direction in allowed_movements.iter() {
            let next = current + *direction;
            let new_cost = cost_so_far[&current] + obstacles[next] + 1;
            if (!true_collidables[next].is_some() && (!cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next])) &&
                (next.x >= 0 && next.x <= collidables.width() as i32 && next.y >= 0 && next.y < collidables.height() as i32) {
                cost_so_far.insert(next, new_cost);
                let priority = new_cost;
                frontier.push(Reverse(WeightedPosition{position: next, weight: priority}));
                came_from.insert(next, Some(current));
            }
            // Early exit
            if next == *goal {
                break 'full;
            }
        }
    }

    let mut current = *goal;
    let mut path = VecDeque::<IVec2>::new();

    while current != *start {
        path.push_front(current);
        if came_from.contains_key(&current) {
            current = came_from[&current].expect("Path unexpectedly empty!");
        } else {
            println!("No path found from {:?} to {:?}.", start, goal);
            path.clear();
            break
        }
    }
    
    if cost_so_far.contains_key(&goal) {
        Path {positions: path, cost: cost_so_far[&goal]}
    } else {
        Path {positions: path, cost: u32::MAX}
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