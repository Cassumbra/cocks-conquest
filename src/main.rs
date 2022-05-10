// Working title: Cock's Conquest (Cocklike)

//#![windows_subsystem = "windows"]

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;
//use bevy_inspector_egui::WorldInspectorPlugin;

mod data;
use data::*;

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
    Setup, MapGen, SpawnActors, FinishSetup,
    Playing,
    Restart,
}

fn main () {
    let mut setup = SystemStage::parallel();
    setup.add_system(setup::setup);

    let mut map_gen = SystemStage::parallel();
    map_gen.add_system(map::entity_map_rooms_passages);

    let mut spawn_actors = SystemStage::parallel();
    spawn_actors.add_system(actors::setup_actors.label("setup_actors"));
    spawn_actors.add_system(rendering::update_render_order.after("setup_actors"));
    spawn_actors.add_system(movement::update_collidables.after("setup_actors"));

    let mut finish_setup = SystemStage::parallel();
    finish_setup.add_system(vision::setup_vision);

    let mut restart = SystemStage::parallel();
    restart.add_system(setup::restart);
    

    App::new()

    .insert_resource(WindowDescriptor{
        title: "Cock's Conquest".to_string(),
        resizable: false,
        ..Default::default()}
    )

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

    .add_state(GameState::Setup)

    
    .add_stage(
        "transition_stage", 
        StateTransitionStage::new(GameState::Setup)
            .with_enter_stage(GameState::Setup, setup)
            .with_enter_stage(GameState::MapGen, map_gen)
            .with_enter_stage(GameState::SpawnActors, spawn_actors)
            .with_enter_stage(GameState::FinishSetup, finish_setup)

            .with_enter_stage(GameState::Restart, restart)
    )

    .add_system(rendering::update_render_order.run_in_state(GameState::Playing))
    .add_system(window::change_size.run_in_state(GameState::Playing))

    
    .add_system_set(
        SystemSet::new()
            .label("rendering")
            .with_system(rendering::render_level_view.run_in_state(GameState::Playing).before("finish_rendering"))
            .with_system(rendering::render_stats_and_log.run_in_state(GameState::Playing).before("finish_rendering"))
            .with_system(rendering::finish_render.run_in_state(GameState::Playing).label("finish_rendering"))
    )
    

    // TODO: Perhaps these should be part of the PostUpdate stage.
    .add_system(turn::update_turn_order.run_in_state(GameState::Playing).label("update_turn_order"))
    .add_system(turn::update_turn.run_in_state(GameState::Playing).label("update_turn").after("update_turn_order"))

    
    .add_system_set(
        SystemSet::new()
            .label("actor_turn")
            .after("update_turn")
            .with_system(ai::generic_brain.run_in_state(GameState::Playing))
            .with_system(ai::tranced_brain.run_in_state(GameState::Playing))
            .with_system(player::player_input_game.run_in_state(GameState::Playing))
            .with_system(player::player_input_meta.run_in_state(GameState::Playing))
    )
    .add_system_set(
        SystemSet::new()
            .label("actor_actions")
            .after("actor_turn")
            .with_system(movement::do_point_move.run_in_state(GameState::Playing))
            .with_system(healing::heal_action.run_in_state(GameState::Playing))
            
    )
    .add_system_set(
        SystemSet::new()
            .label("action_effects")
            .after("actor_actions")
            // Melee attacks are curently an effect of bumping (see point move)
            .with_system(interactions::melee_attack.run_in_state(GameState::Playing).label("melee_attack"))
            .with_system(vore::vore_attack.run_in_state(GameState::Playing).label("vore_attack").after("melee_attack"))
            
    )
    
    .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .with_system(vore::update_vore.run_in_state(GameState::Playing).label("update_vore").before("do_stat_change"))
            .with_system(stats::do_stat_change.run_in_state(GameState::Playing).label("do_stat_change"))
            .with_system(stats::update_fatal.run_in_state(GameState::Playing).label("update_fatal").after("do_stat_change"))
            .with_system(player::player_victory.run_in_state(GameState::Playing).label("player_victory").after("update_fatal"))
            .with_system(player::player_death.run_in_state(GameState::Playing).label("player_death").after("update_fatal"))
            .with_system(movement::update_collidables.run_in_state(GameState::Playing).label("update_collidables"))
            .with_system(vision::update_vision.run_in_state(GameState::Playing).label("update_vision").after("update_collidables"))
            .with_system(vision::update_mind_map.run_in_state(GameState::Playing).after("update_vision"))
    )

    .run();
}