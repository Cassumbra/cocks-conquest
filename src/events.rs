// Unglob later
use bevy::prelude::*;

pub struct CollidableChangeEvent {
    pub old_position: IVec2,
    pub new_position: IVec2,
    pub entity: Entity,
}

pub struct BumpEvent {
    pub bumping_entity: Entity,
    pub bumped_entity: Entity,
}

// We may make a "LineMoveEvent" later.
pub struct PointMoveEvent {
    pub entity: Entity,
    pub movement: IVec2,
}

pub struct EndTurnEvent;

pub struct StartTurnEvent;