use bevy::prelude::*;

use crate::{data::Position, actors::{vision::Vision, TakesTurns, alignments::Relations, stats::Stats, ActorRemovedEvent}, turn::Turns};

use super::Path;

// Components
// Warning: Engagements with distances higher than 1.5 will behave strangely if there is no capacity to perform ranged attacks.
#[derive(Component, Default, Clone)]
pub struct Engages{
    pub target: Option<Entity>,
    pub distance: f32,
    pub path: Path,
}

pub fn targetting_behavior (
    mut turns: ResMut<Turns>,

    mut ai_query: Query<(&Position, &mut Engages, &Relations, &Vision), With<TakesTurns>>,
    actor_query: Query<(Entity, &Position, &Relations), (With<TakesTurns>)>,

    mut ev_actor_removed: EventReader<ActorRemovedEvent>,
) {
    let ai_ent = turns.order[turns.current];
    if let Ok((pos, mut engagement, relations, vision)) = ai_query.get_mut(ai_ent) {
        let mut rng = rand::thread_rng();

        // Remove target if it is no longer an actor
        // TODO: THIS DOES NOT WORK!!!
        //       The event passes before it gets a chance to be useful.
        //       We need to create a special event type which persists for a turn and the turn afterwards.
        if engagement.target.is_some() {
            for ev in ev_actor_removed.iter() {
                println!("woah!");
                if engagement.target.unwrap() == ev.removed_actor {
                    engagement.target = None;
                    break;
                }
            }
        }

        // Stop if we already have a target
        if engagement.target.is_some() {
            return;
        }

        let mut closest_visible_enemy: (Option<Entity>, f32) = (None, f32::MAX);

        'enemy_check: for (actor, actor_pos, actor_relations) in actor_query.iter() {
            // Check if actor is our enemy
            for alignment in &actor_relations.alignments {
                if !relations.enemies.contains(alignment) {
                    continue 'enemy_check;
                }
            }

            // Check if actor is visible
            if !vision.visible(**actor_pos) {
                continue 'enemy_check;
            }

            // Get distance between the enemy and us.
            let distance = actor_pos.as_vec2().distance(pos.as_vec2());

            // Replace the currently closest enemy with this enemy if its closer to us.
            if distance < closest_visible_enemy.1 {
                closest_visible_enemy = (Some(actor), distance);
            }
        }

        // Set our target. (Or None)
        engagement.target = closest_visible_enemy.0;

        // TODO: We can either check all visible tiles for our enemies, OR
        //       we can check for all enemies to see if any are visible.
        //       For the moment, we should do the latter.
        //       If we ever get to having a very large amount of actors, it might be good to switch to the former.
        //       But then we'll be fucked anyways so,


    }
}