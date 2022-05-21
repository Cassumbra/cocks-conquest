use bevy::prelude::*;

use crate::{data::Position, actors::{vision::Vision, TakesTurns, alignments::Relations, stats::Stats}, turn::Turns, actions::interactions::ActorRemovedEvent};

// Components
#[derive(Component, Default, Deref, DerefMut, Clone)]
pub struct Target(Option<Entity>);

pub fn targetting_behavior (
    mut turns: ResMut<Turns>,

    mut ai_query: Query<(&Position, &mut Target, &Relations, &Vision), With<TakesTurns>>,
    actor_query: Query<(Entity, &Position, &Relations), (With<TakesTurns>)>,

    mut ev_actor_removed: EventReader<ActorRemovedEvent>,
) {
    let ai_ent = turns.order[turns.current];
    if let Ok((pos, mut target, relations, vision)) = ai_query.get_mut(ai_ent) {
        let mut rng = rand::thread_rng();

        // Remove target if it is no longer an actor
        if target.is_some() {
            for ev in ev_actor_removed.iter() {
                if target.unwrap() == ev.removed_actor {
                    **target = None;
                    break;
                }
            }
        }

        // Stop if we already have a target
        if target.0.is_some() {
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
            let distance = Vec2::new(actor_pos.x as f32, actor_pos.y as f32).distance_squared(Vec2::new(pos.x as f32, pos.y as f32));

            // Replace the currently closest enemy with this enemy if its closer to us.
            if distance < closest_visible_enemy.1 {
                closest_visible_enemy = (Some(actor), distance);
            }
        }

        // Set our target. (Or None)
        target.0 = closest_visible_enemy.0;

        // TODO: We can either check all visible tiles for our enemies, OR
        //       we can check for all enemies to see if any are visible.
        //       For the moment, we should do the latter.
        //       If we ever get to having a very large amount of actors, it might be good to switch to the former.
        //       But then we'll be fucked anyways so,


    }
}