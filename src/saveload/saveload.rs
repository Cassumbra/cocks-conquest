use bevy::app::AppExit;
use bevy::prelude::*;

// Systems
pub fn save (

) {

}

pub fn quit (
    mut ev_exit: EventWriter<AppExit>,
) {
    ev_exit.send(AppExit);
}

pub fn load (

) {

}