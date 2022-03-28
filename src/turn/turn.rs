use bevy::prelude::*;
use crate::actions::interactions::ActorRemovedEvent;

use super::*;

// Plugin
#[derive(Default)]
pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Turns>();
    }
}

// Resources
#[derive(Default)]
pub struct Turns {
    pub order: Vec<Entity>,
    pub current: usize,
    pub progress: bool,
}
impl Turns {
    pub fn is_turn(&self, entity: &Entity) -> bool {
        //println!("{:?}", self.order);
        self.order.len() > 0 && self.order[self.current] == *entity && !self.progress
    }
    pub fn was_turn(&self, entity: &Entity) -> bool {
        self.order.len() > 0 && self.order[self.current] == *entity && self.progress
    }
    pub fn progress_turn(&mut self) {
        self.progress = true;
    }

}

// Systems
/// Updates the order in which entities take turns.
/// Only gets updated when necessary.
pub fn update_turn_order(
    query: Query<Entity, With<TakesTurns>>,
    turns_changed: Query<(Entity, &TakesTurns), Or<(Changed<TakesTurns>, Added<TakesTurns>)>>,

    mut ev_actor_remove_event: EventReader<ActorRemovedEvent>,

    mut turns: ResMut<Turns>,
) {

    if turns_changed.iter().next().is_some() || 
       ev_actor_remove_event.iter().next().is_some()
    {
        turns.order = Vec::new();
        for ent in query.iter() {
            turns.order.push(ent);
        }
    }
}

pub fn update_turn(
    mut turns: ResMut<Turns>,
) {
    if turns.progress {

        let mut next_turn = turns.current + 1;
        if next_turn > turns.order.len() - 1 {
            next_turn = 0;
        }

        turns.current = next_turn;
        turns.progress = false;
    }
}