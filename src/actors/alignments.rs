use bevy::{prelude::*, utils::HashSet};

// Data
#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Alignment {
    Cock,
    AntiCock
}

// Components
#[derive(Component, Clone)]
pub struct Relations {
    pub alignments: HashSet<Alignment>,
    pub friends: HashSet<Alignment>,
    pub enemies: HashSet<Alignment>,
}
impl Relations {
    pub fn new(alignments: Vec<Alignment>, friends: Vec<Alignment>, enemies: Vec<Alignment>) -> Self {
        Relations { alignments: HashSet::from_iter(alignments), friends: HashSet::from_iter(friends), enemies: HashSet::from_iter(enemies) }
    }
}

pub struct Enemies(HashSet<Entity>);

pub struct Leader(HashSet<Entity>);

pub struct Follows(Entity);