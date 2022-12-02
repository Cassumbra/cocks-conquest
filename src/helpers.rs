use std::thread::current;

use bevy::prelude::*;
use iyes_loopless::state::{NextState, CurrentState};

use crate::{GameState, PreviousState};

pub fn change_state (mut commands: Commands, current_state: GameState, next_state: GameState) {
    commands.insert_resource(PreviousState(current_state));
    commands.insert_resource(NextState(next_state));
    
}