use bevy::prelude::*;

use crate::{data::Position, actors::{vision::Vision, TakesTurns, alignments::Relations, stats::{Stats, StatType}, ActorRemovedEvent}, turn::Turns, actions::attack::Dice};

use super::Path;

// Components
// Warning: Engagements with distances higher than 1.5 will behave strangely if there is no capacity to perform ranged attacks.
#[derive(Component, Clone)]
pub struct Engages {
    pub target: Option<Entity>,
    pub delay: u32,
    pub delay_timer: u32,
    pub distance: f32,
    pub path: Path,
}
impl Default for Engages {
    fn default() -> Self {
        Self {
            target: Default::default(),
            delay: 1,
            delay_timer: Default::default(),
            distance: Default::default(),
            path: Default::default()
        }
    }
}
impl Engages {
    // Gives our current target if delay timer is above delay
    pub fn get_target(&self) -> Option<Entity> {
        if self.delay_timer > self.delay {
            self.target
        }
        else {
            None
        }
    }
    pub fn get_alert(&self) -> bool {
        self.target.is_some() && self.delay_timer > 0 && self.delay_timer <= self.delay
    }
}

pub fn targetting_behavior (
    turns: Res<Turns>,

    mut ai_query: Query<(&Position, &mut Engages, &Relations, &Vision), With<TakesTurns>>,
    actor_query: Query<(Entity, &Position, &Relations, Option<&Stats>), (With<TakesTurns>)>,

    mut ev_actor_removed: EventReader<ActorRemovedEvent>,
) {
    let ai_ent = turns.order[turns.current];
    if let Ok((pos, mut engagement, relations, vision)) = ai_query.get_mut(ai_ent) {
        let mut rng = rand::thread_rng();

        // Remove target if it is no longer an actor
        if engagement.target.is_some() {
            for ev in ev_actor_removed.iter() {
                if  engagement.target.unwrap() == ev.removed_actor {
                    engagement.target = None;
                    break;
                }
            }
        }

        // Stop if we already have a target
        if engagement.get_target().is_some() {
            return;
        }

        let mut closest_visible_enemy: (Option<Entity>, f32) = (None, f32::MAX);

        'enemy_check: for (actor, actor_pos, actor_relations, opt_actor_stats) in actor_query.iter() {
            // Check if actor is our enemy
            for alignment in &actor_relations.alignments {
                if !relations.enemies.contains(alignment) {
                    continue 'enemy_check;
                }
            }

            // Check if actor is visible
            if !vision.visible(**actor_pos) {
                // If our enemy is out of sight, decrease the delay timer.
                if engagement.target == Some(actor) {
                    engagement.delay_timer -= 1;
                    // Remove enemy as target if they have not been visible for too long.
                    if engagement.delay_timer == 0 {
                        engagement.target = None;
                    }
                }
                continue 'enemy_check;
            }
            

            // If our enemy is in sight, increase the delay timer.
            if engagement.target == Some(actor)  { //&& engagement.delay != engagement.delay_timer
                engagement.delay_timer += 1;
                return
            }

            // Get distance between the enemy and us.
            let distance = actor_pos.as_vec2().distance(pos.as_vec2());

            // Replace the currently closest enemy with this enemy if its closer to us.
            if distance < closest_visible_enemy.1 {
                if let Some(actor_stats) = opt_actor_stats {
                    if actor_stats.contains_key(&StatType::StealthRange) {
                        if distance <= actor_stats.get_effective(&StatType::StealthRange) as f32 {
                            let d20 = Dice::new("1d20");
                            // TODO: Put perception here.
                            //let roll = d20.total + ;
                            if d20.total <= 15 {
                                continue 'enemy_check;
                            }
                        }
                        else {
                            continue 'enemy_check;
                        }
                    }
                }

                closest_visible_enemy = (Some(actor), distance);
            }
        }

        // Set our target. (Or None)
        engagement.target = closest_visible_enemy.0;

        if engagement.target.is_some() {
            engagement.delay_timer += 1;
        }
        // TODO: We can either check all visible tiles for our enemies, OR
        //       we can check for all enemies to see if any are visible.
        //       For the moment, we should do the latter.
        //       If we ever get to having a very large amount of actors, it might be good to switch to the former.
        //       But then we'll be fucked anyways so,


    }
}