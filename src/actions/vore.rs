use bevy::prelude::*;

use crate::{log::Log, actors::{TakesTurns, stats::{Stats, StatType, StatChangeEvent}, ActorRemovedEvent, status_effects::{StatusEffects, StatusEffectType}}, data::Collides, rendering::Renderable, turn::Turns};

use super::attack::{BumpEvent, Attack};


// Event
#[derive(Clone)]
pub struct VoreAttackEvent{
    pub predator: Entity,
    pub target: IVec2,
}

// Systems
// TODO: Add digestion attack (?)
pub fn digestion_attack (

) {

}

// TODO: Move this and other vore stuff to another file.
pub fn vore_attack (
    mut commands: Commands,

    mut ev_bump: EventReader<BumpEvent>,
    mut ev_vore_attack: EventReader<VoreAttackEvent>,

    prey_query: Query<(&StatusEffects, Option<&Name>)>,
    pred_query: Query<(&DoesVore, Option<&Name>)>,

    mut log: ResMut<Log>,
) {
    for ev in ev_vore_attack.iter() {
        
    }

    for ev in ev_bump.iter() {
        if let Ok((statuses, opt_prey_name)) = prey_query.get(ev.bumped_entity) {
            if !statuses.has_status_effect(&StatusEffectType::Tranced) {
                return
            }

            if let Ok((_doesvore, opt_pred_name)) = pred_query.get(ev.bumping_entity) {
                let prey_name = if opt_prey_name.is_some() {opt_prey_name.unwrap().to_string()} else {ev.bumped_entity.id().to_string()};
                let pred_name = if opt_pred_name.is_some() {opt_pred_name.unwrap().to_string()} else {ev.bumping_entity.id().to_string()};
    
                log.log_string_formatted(format!(" {} devours {}!", pred_name, prey_name), Color::RED);
                commands.entity(ev.bumped_entity)
                    .remove::<Collides>()
                    .remove::<Renderable>()
                    //.remove::<TakesTurns>()
                    .insert(Digesting{
                        turns_to_digest: 4,
                    });
                commands.entity(ev.bumping_entity)
                    .push_children(&[ev.bumped_entity]);
            }
        }
    }
}

pub fn update_vore (
    mut commands: Commands,

    mut prey_query: Query<(&mut Digesting, Option<&Name>)>,
    mut pred_query: Query<(Entity, &mut Stats, Option<&Name>, &Children), With<TakesTurns>>,

    mut ev_actor_remove_event: EventWriter<ActorRemovedEvent>,
    mut ev_stat_change: EventWriter<StatChangeEvent>,

    turns: Res<Turns>,
    mut log: ResMut<Log>,
) {
    for (pred, mut stats, opt_pred_name, meals) in pred_query.iter_mut() {
        
        if turns.was_turn(&pred) {
            for prey in meals.iter() {
                if let Ok((mut digestion, opt_prey_name)) = prey_query.get_mut(*prey) {
                    let prey_name = if opt_prey_name.is_some() {opt_prey_name.unwrap().to_string()} else {prey.id().to_string()};
                    let pred_name = if opt_pred_name.is_some() {opt_pred_name.unwrap().to_string()} else {pred.id().to_string()};

                    digestion.turns_to_digest -= 1;
                    if digestion.turns_to_digest == 0 {
                        // TODO: Add a check to make sure we don't go over the limit
                        log.log_string_formatted(format!(" {} has been melted into 15 cum points worth of stinky smelly goo.", prey_name), Color::GREEN);
                        commands.entity(*prey).despawn();
                        ev_actor_remove_event.send(ActorRemovedEvent::new(*prey, turns.count));
                        ev_stat_change.send(StatChangeEvent::new(StatType::CumPoints, 15, pred));
                    } else {
                        log.log_string_formatted(format!(" {} turns until {} is digested by {}.", digestion.turns_to_digest, prey_name, pred_name), Color::WHITE);
                    }
                } 

            }
        }
    }
}

// Components
// Should be dynamic like how attacks are (at some point but i don't care)
#[derive(Component, Default, Copy, Clone)]
pub struct DoesVore;

#[derive(Component, Default, Copy, Clone)]
pub struct Digesting {
    pub turns_to_digest: u8,
}