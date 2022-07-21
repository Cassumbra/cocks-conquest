use bevy::prelude::*;

use super::stats::StatType;


// Data
#[derive(Clone)]
pub struct ProgressiveStat {
    pub stat: StatType,
    // TODO: Diceroll?
    pub change: i32,
    pub period: u32,
    pub timer: u32,
    // To be used if attempting to change the stat would result in it being changed out of bounds.
    pub backup: Option<Box<ProgressiveStat>>,
}
impl ProgressiveStat {
    //pub fn 
}

// Components
// Changes to stats that occur over time (and potentially under certain conditions)
#[derive(Component, Default, Clone)]
pub struct ProgressiveStats (Vec<ProgressiveStat>);