// Working title: Cock's Conquest (Cocklike)

//#![windows_subsystem = "windows"]

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;
//use bevy_inspector_egui::WorldInspectorPlugin;

mod data;
use data::*;


#[path = "actions/actions.rs"]
mod actions;
use actions::*;

#[path = "actors/actors.rs"]
mod actors;
use actors::*;

#[path = "ai/ai.rs"]
mod ai;
use ai::*;

#[path = "log/log.rs"]
mod log;
use log::*;

#[path ="turn/turn.rs"]
mod turn;
use turn::*;

#[path = "map/map.rs"]
mod map;
use map::*;

#[path = "player/player.rs"]
mod player;
use player::*;

#[path = "rendering/rendering.rs"]
mod rendering;
use rendering::*;

#[path = "setup/setup.rs"]
mod setup;
//use setup::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Setup, MapGen, SpawnActors, FinishSetup,
    Playing, Targetting,
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
    finish_setup.add_system(turn::update_turn_order);
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
    .add_plugin(player::PlayerPlugin)
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
            .label("targetting")
            .with_system(targetting::start_targetting.run_in_state(GameState::Playing))
            .with_system(targetting::targetting.run_in_state(GameState::Targetting))
    )

    .add_system_set(
        SystemSet::new()
        .label("targetting_rendering")
        .with_system(rendering::render_level_view.run_in_state(GameState::Targetting).label("targetting_render_level").before("targetting_finish_rendering"))
        .with_system(rendering::render_targetting.run_in_state(GameState::Targetting).after("targetting_render_level").before("targetting_finish_rendering"))
        // TODO: It would be cool if we has a special system here that would give information on the actor that is selected
        .with_system(rendering::render_stats_and_log.run_in_state(GameState::Targetting).before("targetting_finish_rendering"))
        .with_system(rendering::render_actor_info.run_in_state(GameState::Targetting).before("targetting_finish_rendering"))
        .with_system(rendering::finish_render.run_in_state(GameState::Targetting).label("targetting_finish_rendering"))
    )

    .add_system_set(
        SystemSet::new()
            .label("rendering")
            .with_system(rendering::render_level_view.run_in_state(GameState::Playing).label("render_level").before("finish_rendering"))
            .with_system(effects::render_effects.run_in_state(GameState::Playing).after("render_level"))
            .with_system(rendering::render_stats_and_log.run_in_state(GameState::Playing).before("finish_rendering"))
            .with_system(rendering::render_actor_info.run_in_state(GameState::Playing).before("finish_rendering"))
            .with_system(rendering::finish_render.run_in_state(GameState::Playing).label("finish_rendering"))       
    )


    
    .add_system_set(
        SystemSet::new()
            .label("actor_turn")
            .with_system(targetting_behavior::targetting_behavior.run_in_state(GameState::Playing).label("targetting"))
            .with_system(status_effects::tranced_behavior.run_in_state(GameState::Playing).label("trance"))
            .with_system(engage_behavior::engage_behavior.run_in_state(GameState::Playing).label("engage").after("trance").after("targetting"))
            .with_system(melee_behavior::melee_behavior.run_in_state(GameState::Playing).label("melee").after("engage"))
            .with_system(ranged_behavior::ranged_behavior.run_in_state(GameState::Playing).label("ranged").after("melee"))
            .with_system(wander_behavior::wander_behavior.run_in_state(GameState::Playing).after("ranged"))
            //.with_system(ai::generic_brain.run_in_state(GameState::Playing))
            //.with_system(ai::tranced_brain.run_in_state(GameState::Playing))
            .with_system(player::player_input_game.run_in_state(GameState::Playing))
            .with_system(player::player_receive_targetting.run_in_state(GameState::Playing))
            .with_system(player::player_input_meta.run_in_state(GameState::Playing))
    )
    .add_system_set(
        SystemSet::new()
            .label("actor_actions")
            .after("actor_turn")
            .with_system(movement::do_point_move.run_in_state(GameState::Playing))
            .with_system(healing::heal_action.run_in_state(GameState::Playing))
            .with_system(ranged::ranged_attack.run_in_state(GameState::Playing))
            .with_system(ranged::rand_ranged_attack.run_in_state(GameState::Playing))
    )
    .add_system_set(
        SystemSet::new()
            .label("action_effects")
            .after("actor_actions")
            // Melee attacks are curently an effect of bumping (see point move)
            .with_system(melee::bump_melee_attack.run_in_state(GameState::Playing).label("melee_attack"))
            .with_system(attack::attack_hit.run_in_state(GameState::Playing).label("attack_hit").after("melee_attack"))
            .with_system(vore::vore_attack.run_in_state(GameState::Playing).label("vore_attack").after("attack_hit"))
            
    )
    
    .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .with_system(vore::update_vore.run_in_state(GameState::Playing).label("update_vore").before("do_stat_change"))
            .with_system(stats::do_stat_change.run_in_state(GameState::Playing).label("do_stat_change"))
            .with_system(status_effects::update_status_effects.run_in_state(GameState::Playing).before("apply_status_effects"))
            .with_system(status_effects::apply_status_effects.run_in_state(GameState::Playing).label("apply_status_effects").before("update_effective_stats"))
            .with_system(stats::update_effective_stats.run_in_state(GameState::Playing).label("update_effective_stats").after("do_stat_change"))
            .with_system(stats::update_fatal.run_in_state(GameState::Playing).label("update_fatal").after("update_effective_stats"))
            .with_system(ending::player_victory.run_in_state(GameState::Playing).label("player_victory").after("update_fatal"))
            .with_system(ending::player_death.run_in_state(GameState::Playing).label("player_death").after("update_fatal"))
            .with_system(movement::update_collidables.run_in_state(GameState::Playing).label("update_collidables"))
            .with_system(vision::update_vision.run_in_state(GameState::Playing).label("update_vision").after("update_collidables"))
            .with_system(vision::update_mind_map.run_in_state(GameState::Playing).after("update_vision"))
    )

    .add_system_set_to_stage (
        CoreStage::Last,
        SystemSet::new()
            .with_system(turn::update_turn_order.run_in_state(GameState::Playing).label("update_turn_order"))
            .with_system(turn::update_turn.run_in_state(GameState::Playing).label("update_turn").after("update_turn_order"))
            .with_system(turn::turn_event_manager::<ActorRemovedEvent>.after("update_turn"))
    )

    .run();
}