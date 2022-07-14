use bevy::prelude::*;

use super::stats::StatType;

// Data
#[derive(Clone)]
pub enum TempStatReason {
    None,
    Wait,
    RangedAttack,
    MeleeAttack,
}

/*
#[derive(Clone)]
pub struct TempStat {
    pub stat_type: StatType,
    pub value: i32,
    pub duration: u32,
}
impl Stat {
    pub fn new(min: i32, max: i32, visibility: StatVisibility) -> Stat {
        Stat {value: max, min, max, visibility}
    }

    pub fn with_value(value: i32, min: i32, max: i32, visibility: StatVisibility) -> Stat {
        Stat {value, min, max, visibility}
    }
}

// Components
#[derive(Component, Clone, Deref, DerefMut)]
pub struct TempStats(pub Vec<TempStat>);
impl TempStats {
    pub fn push_or_refresh(&mut self, temp_stat: TempStat) {

    }

    //pub fn push_no_dupe(&mut self, temp_stat: TempStat) {
    //    TempStats.contains()
    //}
}
 */