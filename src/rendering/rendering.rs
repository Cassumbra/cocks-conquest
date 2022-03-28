use bevy::prelude::*;
use bevy_ascii_terminal::{Tile, Terminal};
use inflector::Inflector;
use crate::actors::{player::Player, vision::{Vision, MindMap}};
use crate::actors::stats::Stats;

use super::*;

pub mod window;

//Plugin
#[derive(Default)]
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<RenderOrder>()
        .init_resource::<BottomSize>();
    }
}

//Components
#[derive(Component, Default, Copy, Clone)]
pub struct Renderable {
    pub tile: Tile,
    pub order: u8,
}

//Resources
#[derive(Default)]
pub struct RenderOrder(pub Vec<Entity>);

// We need to make a more sophisticated way for doing this the next time we do this.
// This should work for now, though.

// TODO: When remaking this, create display entities with width and height. Have a ScreenSize resource.
// Display entities are parents of entities with a renderable component.
// ScreenSize determines the max size of display entities.
// Display entities can be made with a rectangle component and a display tag component.
// Future me: Take these plans at your own discretion. Also, don't bother working on any of this shit if you're not even future me.
pub struct BottomSize {
    pub height: u32,
}
impl Default for BottomSize {
    fn default() -> BottomSize {
        BottomSize {
            height: 10,
        }
    }
}

//Systems
/// Updates the order in which entities are drawn.
/// Only gets updated when necessary.
pub fn update_render_order(
    mut commands: Commands,
    query: Query<(Entity, &Renderable, &Position)>,
    renderable_changed: Query<(&Renderable), Or<(Changed<Renderable>, Added<Renderable>)>>,
) {
    if renderable_changed.iter().next().is_some() {
        let mut vec = query.iter().collect::<Vec<(Entity, &Renderable, &Position)>>();
        vec.sort_by_key(|e| e.1.order);
        let entities = vec.into_iter()
            .map(|tuple| tuple.0)
            .collect::<Vec<Entity>>();
        commands.insert_resource(RenderOrder(entities));
    }
}

/// Rendering system.
/// Renders all entities with render and position components.
pub fn render (
    query: Query<(&Renderable, &Position)>,
    player_query: Query<(&Vision, &MindMap), With<Player>>,
    mut term_query: Query<&mut Terminal>,

    order: Res<RenderOrder>,
    bottom_size: Res<BottomSize>,
) {
    let mut terminal = term_query.single_mut();
    let (vis, mind_map) = player_query.single();

    //this is NOT GOOD
    //but clearing the terminal results in screwiness soooo
    //terminal.clear();
    
    for (index, position) in mind_map.seen.iter_2d() {
        for (entity, tile) in position {
            let i_pos_x = index.x as i32;
            let i_pos_y = index.y + bottom_size.height as i32;

            let new_tile = Tile {
                glyph: tile.glyph,
                fg_color: change_brightness(greyscale(tile.fg_color), -0.90),
                bg_color: tile.bg_color //change_brightness(greyscale(tile.bg_color), -0.90),
            };
        
            terminal.put_tile([i_pos_x, i_pos_y], new_tile)
        }
    }

    for e in order.0.iter() {
        if let Ok((rend, pos)) = query.get(*e) {
            if vis.0.visible[pos.0] {
                let i_pos_x = pos.0.x as i32;
                let i_pos_y = pos.0.y + bottom_size.height as i32;
                
                let tile = rend.tile;
                
                let current_tile = terminal.get_tile([i_pos_x, i_pos_y]);

                if tile.bg_color.a() == 1.0 {
                    terminal.put_tile([i_pos_x, i_pos_y], tile);
                }
                else if tile.bg_color.a() == 0.0 {
                    let new_tile = Tile {
                        glyph: tile.glyph,
                        fg_color: tile.fg_color,
                        bg_color: current_tile.bg_color,
                    };
        
                    terminal.put_tile([i_pos_x, i_pos_y], new_tile);
                }
                else {
                    let new_tile = Tile {
                        glyph: tile.glyph,
                        fg_color: tile.fg_color,
                        bg_color: blend_colors(tile.bg_color, current_tile.bg_color),
                    };
                    terminal.put_tile([i_pos_x, i_pos_y], new_tile);
                }
            }
        }
    }
}

pub fn render_stats (
    player_query: Query<(Entity, &Stats, Option<&Name>), With<Player>>,
    mut term_query: Query<&mut Terminal>,

    bottom_size: Res<BottomSize>,
) {
    let mut terminal = term_query.single_mut();

    let (player, stats, opt_name) = player_query.single();

    let mut name = player.id().to_string();

    if let Some(temp_name) = opt_name {
        name = temp_name.to_string();
    }
    
    let print_string = format![" {}    ", &name];
    terminal.put_string([1, bottom_size.height as i32], &print_string);

    let mut current_length = print_string.len();

    for stat in stats.0.iter() {
        let print_string = format!["{}: {}  ", stat.0.to_title_case(), stat.1];
        terminal.put_string([current_length as i32, bottom_size.height as i32], &print_string);
        current_length += print_string.len();
    }
}

/// Blends two color components using their alpha values and a new alpha value.
/// "1" overlaps "2".
fn blend_color_component(a1: f32, a2: f32, a3: f32, c1: f32, c2: f32) -> f32 {
    ((c1 * a1) + ((c2 * a2) * ( 1.0 - a1))) / a3
}

/// Blends two colors.
/// "1" overlaps "2"
fn blend_colors(color1: Color, color2: Color) -> Color {
    let new_alpha = color1.a() + (color2.a() * (1.0 - color1.a()));

    let new_red = blend_color_component(color1.a(), color2.a(), new_alpha, color1.r(), color2.r());
    let new_green = blend_color_component(color1.a(), color2.a(), new_alpha, color1.g(), color2.g());
    let new_blue = blend_color_component(color1.a(), color2.a(), new_alpha, color1.b(), color2.b());

    Color::rgba(new_red, new_green, new_blue, new_alpha)
}

fn greyscale(color: Color) -> Color {
    let brightness = (color.r() + color.g() + color.b()) / 3.0;
    Color::rgba(brightness, brightness, brightness, color.a())
}

// Modifies a color's brightness by some percentage.
fn change_brightness(color: Color, amount: f32) -> Color {
    let new_red = (color.r() + (color.r() * amount))
        .clamp(0.0, 1.0);
    let new_green = (color.g() + (color.g() * amount))
        .clamp(0.0, 1.0);
    let new_blue = (color.b() + (color.b() * amount))
        .clamp(0.0, 1.0);

    Color::rgba(new_red, new_green, new_blue, color.a())
}