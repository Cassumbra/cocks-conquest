// Working title: Cock's Conquest (Cocklike)

use bevy::prelude::*;
use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;
//use bevy_inspector_egui::WorldInspectorPlugin;

mod components;
use components::*;

// The pluginification!
#[path = "actors/actors.rs"]
mod actors;
use actors::*;

#[path = "actions/actions.rs"]
mod actions;
use actions::*;

#[path ="turn/turn.rs"]
mod turn;
use turn::*;

#[path = "map/map.rs"]
mod map;
use map::*;

#[path = "rendering/rendering.rs"]
mod rendering;
use rendering::*;

#[path = "setup/setup.rs"]
mod setup;
//use setup::*;


fn main () {
    App::new()

    .add_plugins(DefaultPlugins)
    .add_plugin(TerminalPlugin)
    .add_plugin(TiledCameraPlugin)
    //.add_plugin(WorldInspectorPlugin::new())

    .add_plugin(actions::ActionPlugin)
    .add_plugin(window::WindowPlugin)
    .add_plugin(rendering::RenderingPlugin)
    .add_plugin(turn::TurnPlugin)
    .add_plugin(movement::MovementPlugin)
    .add_plugin(map::MapPlugin)

    .add_startup_stage("setup", SystemStage::parallel())
    .add_startup_stage_after("setup", "map_gen", SystemStage::parallel())
    .add_startup_stage_after("map_gen", "actor_placement", SystemStage::parallel())
    .add_startup_stage_after("actor_placement", "setup_vision", SystemStage::parallel())
    
    .add_startup_system_to_stage("setup", setup::setup)
    .add_startup_system_to_stage("map_gen", map::entity_map_rooms_passages)
    .add_startup_system_to_stage("actor_placement", actors::setup_actors)
    .add_startup_system_to_stage("setup_vision", actors::setup_vision)

    .add_system(rendering::update_render_order)
    .add_system(rendering::render)
    .add_system(window::change_size)

    .add_system(turn::update_turn_order.label("update_turn_order"))
    .add_system(turn::update_turn.label("update_turn").after("update_turn_order"))
    .add_system_set(
        SystemSet::new()
            .label("actor_turn")
            .after("update_turn_order")
            .with_system(ai::generic_brain)
            .with_system(player::player_input)
    )
    .add_system(movement::do_point_move.label("movement").after("actor_turn"))
    .add_system(movement::update_collidables.label("update_collidables").after("movement"))
    .add_system(interactions::melee_attack.label("melee_attack").after("movement"))
    .add_system(actors::update_vision.label("update_vision").after("update_collidables"))
    .add_system(actors::update_mind_map.after("update_vision"))

    .run();
}