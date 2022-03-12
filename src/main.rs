// Unglob later
use bevy::prelude::*;
use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod systems;
use systems::*;
mod components;
use components::*;
mod resources;
use resources::*;
mod events;
use events::*;
mod bundles;
use bundles::*;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum RunState {
    Loading,
    Ready,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Setup, MapGen, PlayerPlacement,
    Playing,
}

fn main () {
    App::new()

    .add_plugins(DefaultPlugins)
    .add_plugin(TerminalPlugin)
    .add_plugin(TiledCameraPlugin)
    .add_plugin(WorldInspectorPlugin::new())

    .init_resource::<RenderOrder>()
    .init_resource::<MapSize>()
    .init_resource::<BottomSize>()
    .init_resource::<SpriteMagnification>()
    .init_resource::<Collidables>()
    .init_resource::<Turns>()
    .init_resource::<Rooms>()

    .add_event::<CollidableChangeEvent>()
    .add_event::<PointMoveEvent>()

    //.insert_resource::<GameState>(GameState::Setup)

    .add_state(GameState::Setup)
    //.add_state(RunState::Loading)

    
    .add_startup_stage("setup", SystemStage::parallel())
    .add_startup_system_to_stage("setup", setup::setup)

    .add_startup_stage_after("setup", "map_gen", SystemStage::parallel())
    .add_startup_system_to_stage("map_gen", map::entity_map_rooms_passages)

    .add_startup_stage_after("map_gen", "actor_placement", SystemStage::parallel())
    .add_startup_system_to_stage("actor_placement", setup::setup_actors)

    
    .add_stage("pre_update", SystemStage::parallel())
    .add_system_to_stage("pre_update", turns::ensure_turn_exists)
    .add_system_to_stage("pre_update", turns::update_turn_order)
    .add_system_to_stage("pre_update", turns::update_turn)
    .add_system_to_stage("pre_update", movement::update_collidables)
    .add_system_to_stage("pre_update", rendering::update_render_order)

    .add_stage_after("pre_update", "update", SystemStage::parallel())
    .add_system_to_stage("update", player_input::player_input)
    .add_system_to_stage("update", ai::walk_at_player)
    .add_system_to_stage("update", rendering::render)

    .add_stage_after("update", "post_update", SystemStage::parallel())
    .add_system_to_stage("post_update", movement::do_point_move)
    

    /*
    .add_system_set(
        SystemSet::on_update(GameState::Setup)
            .with_system(setup::setup))

    .add_system_set(
        SystemSet::on_update(GameState::MapGen)
            .with_system(map::entity_map_rooms_passages))

    .add_system_set(
        SystemSet::on_enter(GameState::PlayerPlacement)
            .with_system(setup::setup_actors))

    
    
    .add_system_set(
        SystemSet::on_update(GameState::Playing)
            .with_system(turns::ensure_turn_exists)
            .with_system(turns::update_turn_order)
            .with_system(turns::update_turn)
            .with_system(movement::update_collidables)
            .with_system(rendering::update_render_order)

            .with_system(player_input::player_input)
            .with_system(ai::walk_at_player)
            .with_system(movement::do_movements)
            .with_system(rendering::render)
    )
    */

    .run();
}