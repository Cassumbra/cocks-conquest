use bevy::prelude::*;
use bevy_ascii_terminal::{Tile, Terminal};
use inflector::Inflector;
use crate::{actors::{vision::{Vision, MindMap}}, player::targetting::Targetting, actions::ranged::get_line_points};
use crate::actors::stats::Stats;

use super::*;

pub mod window;
pub mod effects;

//Plugin
#[derive(Default)]
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<RenderOrder>()
        .init_resource::<BottomSize>()
        .init_resource::<TemporaryTerminal>();
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

#[derive(Default)]
pub struct TemporaryTerminal(pub Terminal);

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

pub fn finish_render (
    mut temporary_terminal: ResMut<TemporaryTerminal>,

    mut terminal_query: Query<&mut Terminal>,
) {
    let mut terminal = terminal_query.single_mut();

    terminal.tiles = temporary_terminal.0.tiles.clone();
    
    temporary_terminal.0 = Terminal::with_size([terminal.width(), terminal.height()]);
}

pub fn render_level_view (
    query: Query<(&Renderable, &Position)>,
    player_query: Query<(&Vision, &MindMap), With<Player>>,

    order: Res<RenderOrder>,
    bottom_size: Res<BottomSize>,
    mut terminal: ResMut<TemporaryTerminal>,
) {
    let (vis, mind_map) = player_query.single();
    
    for (index, position) in mind_map.seen.iter_2d() {
        for (entity, tile) in position {
            let i_pos_x = index.x as i32;
            let i_pos_y = index.y + bottom_size.height as i32;

            let new_tile = Tile {
                glyph: tile.glyph,
                fg_color: change_brightness(greyscale(tile.fg_color), -0.90),
                bg_color: tile.bg_color //change_brightness(greyscale(tile.bg_color), -0.90),
            };
        
            terminal.0.put_tile([i_pos_x, i_pos_y], new_tile)
        }
    }

    for e in order.0.iter() {
        if let Ok((rend, pos)) = query.get(*e) {
            if vis.0.visible[pos.0] {
                let i_pos_x = pos.0.x as i32;
                let i_pos_y = pos.0.y + bottom_size.height as i32;
                
                let tile = rend.tile;
                
                let current_tile = terminal.0.get_tile([i_pos_x, i_pos_y]);

                if tile.bg_color.a() == 1.0 {
                    terminal.0.put_tile([i_pos_x, i_pos_y], tile);
                }
                else if tile.bg_color.a() == 0.0 {
                    let new_tile = Tile {
                        glyph: tile.glyph,
                        fg_color: tile.fg_color,
                        bg_color: current_tile.bg_color,
                    };
        
                    terminal.0.put_tile([i_pos_x, i_pos_y], new_tile);
                }
                else {
                    let new_tile = Tile {
                        glyph: tile.glyph,
                        fg_color: tile.fg_color,
                        bg_color: blend_colors(tile.bg_color, current_tile.bg_color),
                    };
                    terminal.0.put_tile([i_pos_x, i_pos_y], new_tile);
                }
            }
        }
    }
}

pub fn render_stats_and_log (
    player_query: Query<(Entity, &Stats, Option<&Name>), With<Player>>,

    bottom_size: Res<BottomSize>,
    log: Res<Log>,
    mut terminal: ResMut<TemporaryTerminal>,
) {
    let (player, stats, opt_name) = player_query.single();

    let mut name = player.id().to_string();

    if let Some(temp_name) = opt_name {
        name = temp_name.to_string();
    }
    
    let mut print_strings = vec![format![" {}    ", &name]];

    for stat in stats.0.iter() {
        print_strings.push(format!["{}: {}  ", stat.0.to_string().to_title_case(), stat.1.value]);
    }
    let [mut current_length, mut current_line] = put_string_vec([0, (bottom_size.height-1) as i32], &print_strings, &mut terminal.0);

    // Log rendering
    let lines: &[Vec<LogFragment>];

    if log.lines.len() < bottom_size.height as usize {
        lines = &log.lines[..];
    } else {
        lines = &log.lines[log.lines.len()-bottom_size.height as usize..log.lines.len()]
    }

    for line in lines.iter().rev() {
        current_line -= 1;
        [current_length, current_line] = put_string_vec_formatted([1, current_line], line, &mut terminal.0);
    }
}

pub fn render_targetting (
    bottom_size: Res<BottomSize>,
    targetting: Res<Targetting>,
    mut terminal: ResMut<TemporaryTerminal>,
) {
    // TODO: draw line
    let distance = targetting.position.as_vec2().distance(targetting.target.as_vec2());

    let mut points = get_line_points(targetting.position.as_vec2(), targetting.target.as_vec2(), distance);

    points.pop_front();
    points.push_back(targetting.target);

    for (i, point) in points.iter().enumerate() {
        let i_pos_x = point.x as i32;
        let i_pos_y = point.y + bottom_size.height as i32;

        let glyph = if i == points.len() - 1 {'X'} else {'-'};

        let tile = Tile {
            glyph,
            fg_color: Color::WHITE,
            bg_color: Color::BLACK,
        };
        terminal.0.put_tile([i_pos_x, i_pos_y], tile);
    }
}

// Helper Systems
fn put_string_vec (
    position: [i32; 2],
    strings: &Vec<String>,
    terminal: &mut Terminal,
) -> [i32; 2] {
    let [mut current_length, mut current_line] = position;
    for string in strings.iter() {
        if !terminal.is_in_bounds([current_length + string.len() as i32, current_line]) {
            current_length = position[0];
            current_line -= 1;
            if !terminal.is_in_bounds([current_length + string.len() as i32, current_line]) {
                //eprintln!("ERROR: Cannot fit strings in terminal!");
                break;
            }
        }
        
        terminal.put_string([current_length, current_line], string);
        current_length += string.len() as i32;
    }
    [current_length, current_line]
}

fn put_string_vec_formatted (
    position: [i32; 2],
    fragments: &Vec<LogFragment>,
    terminal: &mut Terminal,
) -> [i32; 2] {
    let [mut current_length, mut current_line] = position;
    for fragment in fragments.iter() {
        let string = &fragment.text;

        if string == "\n " {
            
            current_length = position[0];
            current_line -= 1;
            continue;
        }

        if !terminal.is_in_bounds([current_length + string.len() as i32, current_line]) {
            current_length = position[0];
            current_line -= 1;
            if !terminal.is_in_bounds([current_length + string.len() as i32, current_line]) {
                //eprintln!("ERROR: Cannot fit strings in terminal!");
                break;
            }
        }
        
        terminal.put_string_formatted([current_length, current_line], string, StringFormat { pivot: Pivot::BottomLeft, fg_color: fragment.color, bg_color: Color::BLACK });
        current_length += string.len() as i32;
    }
    [current_length, current_line]
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

// TODO: Create helper for inverting color. This will be good for highlighting text as "selected" (probably?)