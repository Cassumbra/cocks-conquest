use std::thread::current;

use bevy::prelude::*;
use iyes_loopless::state::{NextState, CurrentState};

use crate::{GameState, PreviousState};

// Functions
pub fn change_state (mut commands: Commands, current_state: GameState, next_state: GameState) {
    commands.insert_resource(PreviousState(current_state));
    commands.insert_resource(NextState(next_state));
}

// Systems
pub fn reverse_state (
    mut commands: Commands,
    previous_state: Res<PreviousState>
) {
    commands.insert_resource(NextState(*previous_state));
}