// Unglob later
use bevy::{prelude::*, window::WindowId};
use bevy_ascii_terminal::*;
use super::super::*;

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
pub fn render(
    query: Query<(&Renderable, &Position)>,
    mut term_query: Query<&mut Terminal>,
    order: Res<RenderOrder>,
    bottom_size: Res<BottomSize>,
) {
    let mut terminal = term_query.single_mut();
    
    terminal.clear();
    
    //terminal.draw_border_single();

    for e in order.0.iter() {
        if let Ok((rend, pos)) = query.get(*e) {

            let i_pos_x = pos.0.x as i32;
            let i_pos_y = pos.0.y + bottom_size.height as i32;
            
            let current_tile = terminal.get_tile([i_pos_x, i_pos_y]);

            let tile = rend.tile;
            
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
        } else {

        }
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