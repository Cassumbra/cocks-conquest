// Unglob later
use bevy::prelude::*;
//use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;

mod systems;
use systems::*;
mod components;
use components::*;
mod resources;
use resources::*;


fn main () {
    App::new()
    .add_plugins(DefaultPlugins)
    //.add_plugin(TerminalPlugin)
    //.add_plugin(TiledCameraPlugin)
    .add_startup_system(setup.system())
    //.add_system(update_render_order.system())
    //.add_system(render.system())
    .run();
}