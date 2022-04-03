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

#[path = "log/log.rs"]
mod log;
use log::*;

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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Starting,
    Playing,
}

fn main () {
    App::new()

    .add_plugins(DefaultPlugins)
    .add_plugin(TerminalPlugin)
    .add_plugin(TiledCameraPlugin)
    //.add_plugin(WorldInspectorPlugin::new())

    .add_plugin(actors::ActorPlugin)
    .add_plugin(actions::ActionPlugin)
    .add_plugin(log::LogPlugin)
    .add_plugin(window::WindowPlugin)
    .add_plugin(rendering::RenderingPlugin)
    .add_plugin(turn::TurnPlugin)
    .add_plugin(map::MapPlugin)

    .add_state(GameState::Starting)

    
    .add_stage("setup", SystemStage::parallel())
    //.add_stage_after("setup", "map_gen", SystemStage::parallel())
    //.add_stage_after("map_gen", "actor_placement", SystemStage::parallel())
    //.add_stage_after("actor_placement", "setup_vision", SystemStage::parallel())
    //.add_stage_after("setup_vision", "finish_setup", SystemStage::parallel())

    .add_system_set_to_stage(
        "setup",
        //SystemSet::new()
        SystemSet::on_update(GameState::Starting)
            .with_system(setup::setup)
    )

    /*
    .add_system_set_to_stage(
        "map_gen",
        SystemSet::on_update(GameState::Starting)
            .with_system(map::entity_map_rooms_passages)
    )

    .add_system_set_to_stage(
        "actor_placement",
        SystemSet::on_update(GameState::Starting)
            //.with_system(actors::setup_actors.label("setup_actors"))
            //.with_system(rendering::update_render_order.after("setup_actors"))
            //.with_system(movement::update_collidables.after("setup_actors"))
    )

    .add_system_set_to_stage(
        "setup_vision",
        SystemSet::on_update(GameState::Starting)
            .with_system(vision::setup_vision)
    )
    */

    /* 
    .add_system_set_to_stage(
        "finish_setup",
        SystemSet::on_update(GameState::Starting)
            .with_system(setup::finish_setup)
    )
    */

    //.add_startup_system_to_stage("setup", setup::setup)
    //.add_startup_system_to_stage("map_gen", map::entity_map_rooms_passages)
    //.add_startup_system_to_stage("actor_placement", actors::setup_actors.label("setup_actors"))
    //.add_startup_system_to_stage("actor_placement", rendering::update_render_order.after("setup_actors"))
    //.add_startup_system_to_stage("actor_placement", movement::update_collidables.after("setup_actors"))
    //.add_startup_system_to_stage("setup_vision", vision::setup_vision)

    //.add_system(rendering::update_render_order)
    //.add_system(window::change_size)

    /* 
    .add_system_set(
        SystemSet::on_update(GameState::Playing)
            //.label("rendering")
            //.with_system(rendering::render_level_view.before("finish_rendering"))
            //.with_system(rendering::render_stats_and_log.before("finish_rendering"))
            //.with_system(rendering::finish_render.label("finish_rendering"))
    )
    */

    // TODO: Perhaps these should be part of the PostUpdate stage.
    //.add_system(turn::update_turn_order.label("update_turn_order"))
    //.add_system(turn::update_turn.label("update_turn").after("update_turn_order"))

    /*
    .add_system_set(
        SystemSet::on_update(GameState::Playing)
            .label("actor_turn")
            .after("update_turn")
            .with_system(ai::generic_brain)
            .with_system(ai::tranced_brain)
            .with_system(player::player_input_game)
            .with_system(player::player_input_meta)
    )
    .add_system_set(
        SystemSet::on_update(GameState::Playing)
            .label("actor_actions")
            .after("actor_turn")
            .with_system(movement::do_point_move)
            .with_system(interactions::heal_action)
            
    )
    .add_system_set(
        SystemSet::on_update(GameState::Playing)
            .label("action_effects")
            .after("actor_actions")
            // Melee attacks are curently an effect of bumping (see point move)
            .with_system(interactions::melee_attack.label("melee_attack"))
            .with_system(interactions::vore_attack.label("vore_attack").after("melee_attack"))
            
    )
    
    .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::on_update(GameState::Playing)
            .with_system(interactions::update_vore.label("update_vore").before("do_stat_change"))
            .with_system(stats::do_stat_change.label("do_stat_change"))
            .with_system(stats::update_fatal.label("update_fatal").after("do_stat_change"))
            .with_system(player::player_victory.label("player_victory").after("update_fatal"))
            .with_system(player::player_death.label("player_death").after("update_fatal"))
            .with_system(movement::update_collidables.label("update_collidables"))
            .with_system(vision::update_vision.label("update_vision").after("update_collidables"))
            .with_system(vision::update_mind_map.after("update_vision"))
    )
    */

    //.add_system_to_stage(CoreStage::PostUpdate, interactions::update_vore.label("update_vore").before("do_stat_change"))
    //.add_system_to_stage(CoreStage::PostUpdate, stats::do_stat_change.label("do_stat_change"))
    //.add_system_to_stage(CoreStage::PostUpdate, stats::update_fatal.label("update_fatal").after("do_stat_change"))
    //.add_system_to_stage(CoreStage::PostUpdate, player::player_victory.label("player_victory").after("update_fatal"))
    //.add_system_to_stage(CoreStage::PostUpdate, player::player_death.label("player_death").after("update_fatal"))
    //.add_system_to_stage(CoreStage::PostUpdate, movement::update_collidables.label("update_collidables"))
    //.add_system_to_stage(CoreStage::PostUpdate, vision::update_vision.label("update_vision").after("update_collidables"))
    //.add_system_to_stage(CoreStage::PostUpdate, vision::update_mind_map.after("update_vision"))


    .run();
}