// Unglob later
use bevy::prelude::*;
use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;
use bevy_inspector_egui::WorldInspectorPlugin;

mod components;
use components::*;
mod resources;
use resources::*;
mod events;
use events::*;
mod bundles;
use bundles::*;

// The pluginification!
mod setup;
use setup::*;
mod ai;
use ai::*;
mod player;
use player::*;
mod actions;
use actions::*;
mod turn;
use turn::*;
mod map;
use map::*;
mod rendering;
use rendering::*;


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

    .add_startup_stage("setup", SystemStage::parallel())
    .add_startup_stage_after("setup", "map_gen", SystemStage::parallel())
    .add_startup_stage_after("map_gen", "actor_placement", SystemStage::parallel())
    
    .add_stage("pre_update", SystemStage::parallel())
    .add_stage_after("pre_update", "update", SystemStage::parallel())
    .add_stage_after("update", "post_update", SystemStage::parallel())

    
    .add_plugin(SetupPlugin)
    .add_plugin(AIPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(ActionPlugin)
    .add_plugin(TurnPlugin)
    .add_plugin(MapPlugin)
    .add_plugin(RenderingPlugin)


    .run();
}