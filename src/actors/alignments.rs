use bevy::{prelude::*, utils::HashSet};

// Data
pub enum Alignment {
    Cock,
    AntiCock
}

// Components
pub struct Relations {
    pub alignments: HashSet<Alignment>,
    pub friends: HashSet<Alignment>,
    pub enemies: HashSet<Alignment>,
}

pub struct Enemies(HashSet<Entity>);

pub struct Leader(HashSet<Entity>);

pub struct Follows(Entity);