use bevy::{prelude::*, input::{keyboard::KeyboardInput, ElementState}};
use iyes_loopless::state::NextState;

use crate::{GameState, map::MapSize, actions::{ranged::{Projectile, RangedAttackEvent}}, turn::Turns, actors::Moves, data::Position};


// Data
#[derive(Clone)]
pub enum TargetIntent {
    None,
    RangedAttack (Projectile),
    VoreAttack,
}

// Events
pub struct StartTargetEvent{
    pub intent: TargetIntent,
    pub entity: Entity,
    pub position: IVec2,
}
impl StartTargetEvent {
    pub fn new(intent: TargetIntent, entity: Entity, position: IVec2) -> Self {
        Self {intent, entity, position, }
    }
}

pub struct FinishTargetEvent {
    pub intent: TargetIntent,
}

#[derive(Deref, DerefMut)]
pub struct MoveTargetEvent (pub IVec2);

//pub struct FinishTargetEvent<T>(T);

// Resources
pub struct Targetting {
    // TODO: In the future, we should just pass in an event that has a Targettable trait.
    //       Said trait would have functions for targetting to get information it needs.
    //       It would also have functions for setting the values it needs to.
    pub intent: TargetIntent,
    // None seems kinda nonsensical, but we need it to initialize targetting as a default resource.
    pub entity: Entity,
    pub position: IVec2,
    pub target: IVec2,
}
/*
impl Default for Targetting {
    fn default() -> Self {
        Self { intent: TargetIntent::None, position: Default::default(), target: Default::default() }
    }
}  */

// Systems
pub fn start_targetting (
    mut commands: Commands,

    mut ev_start_targetting: EventReader<StartTargetEvent>,
    
    //mut targetting: ResMut<Targetting>,
) {
    if let Some(ev) = ev_start_targetting.iter().next() {
        commands.insert_resource(Targetting{ intent: ev.intent.clone(), entity: ev.entity, position: ev.position, target: ev.position });
        //targetting.intent = ev.intent.clone();
        //targetting.position = ev.position;
        //targetting.target = ev.position;
        commands.insert_resource(NextState(GameState::Targetting));
    }
}

pub fn targetting_controls (
    mut commands: Commands,

    mut ev_key: EventReader<KeyboardInput>,
    mut ev_finish_target: EventWriter<FinishTargetEvent>,
    mut ev_move_target: EventWriter<MoveTargetEvent>,

    map_size: Res<MapSize>,
    mut targetting: ResMut<Targetting>,
    mut turns: ResMut<Turns>,
) {
    for ev in ev_key.iter() {
        if ev.state == ElementState::Pressed {
            match ev.key_code {
                // Cardinal Movement
                Some(KeyCode::I) | Some(KeyCode::Numpad8) => {
                    ev_move_target.send(MoveTargetEvent(IVec2::new(0, 1)));
                }
                Some(KeyCode::Comma) | Some(KeyCode::Numpad2) => {
                    ev_move_target.send(MoveTargetEvent(IVec2::new(0, -1)));
                }
                Some(KeyCode::J) | Some(KeyCode::Numpad4) => {
                    ev_move_target.send(MoveTargetEvent(IVec2::new(-1, 0)));
                }
                Some(KeyCode::L) | Some(KeyCode::Numpad6) => {
                    ev_move_target.send(MoveTargetEvent(IVec2::new(1, 0)));
                }
    
                // Diagonal Movement
                Some(KeyCode::U) | Some(KeyCode::Numpad7) => {
                    ev_move_target.send(MoveTargetEvent(IVec2::new(-1, 1)));
                }
                Some(KeyCode::O) | Some(KeyCode::Numpad9) => {
                    ev_move_target.send(MoveTargetEvent(IVec2::new(1, 1)));
                }
                Some(KeyCode::M) | Some(KeyCode::Numpad1) => {
                    ev_move_target.send(MoveTargetEvent(IVec2::new(-1, -1)));
                }
                Some(KeyCode::Period) | Some(KeyCode::Numpad3) => {
                    ev_move_target.send(MoveTargetEvent(IVec2::new(1, -1)));
                }

                // Select Target
                Some(KeyCode::Return) | Some(KeyCode::Space) | Some(KeyCode::C) => {
                    


                    match &targetting.intent {
                        TargetIntent::RangedAttack(projectile) => {


                            let attack = RangedAttackEvent {
                                targetting_entity: intent.targetting_entity,
                                target: targetting.target,
                                projectile: intent.projectile.clone(),
                            };

                            ev_finish_target.send(FinishTargetEvent { intent: TargetIntent::RangedAttack(attack) });
                        }

                        _ => {

                        }
                    }
                    
                    commands.insert_resource(NextState(GameState::Playing));
                }

                // Cancel Targetting
                Some(KeyCode::Escape) => {
                    commands.insert_resource(NextState(GameState::Playing));
                }

                _ => {}
            }
        }
    }

    targetting.target.x = targetting.target.x.clamp(0, map_size.width as i32 - 1);
    targetting.target.y = targetting.target.y.clamp(0, map_size.height as i32 - 1);
}

/// Ensure movements of targetting reticle are OK
pub fn targetting (
    mut commands: Commands,

    targetter_query: Query<(&Position, Option<&Moves>, )>,

    mut ev_key: EventReader<KeyboardInput>,
    mut ev_finish_target: EventWriter<FinishTargetEvent>,
    mut ev_move_target: EventReader<MoveTargetEvent>,

    map_size: Res<MapSize>,
    mut targetting: ResMut<Targetting>,
    mut turns: ResMut<Turns>,
) {
    //for ev in ev_move_target {
        //let new_pos = 
    //}
    match targetting.intent {
        TargetIntent::None => todo!(),
        TargetIntent::RangedAttack(_) => todo!(),
        TargetIntent::VoreAttack(_) => todo!(),
    }
}