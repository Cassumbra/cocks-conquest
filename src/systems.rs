// Unglob later
use bevy::prelude::*;
use bevy::render::color::*;
use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;
use super::*;

pub fn setup(mut commands: Commands) {
    let size = [20, 20];

    let mut term_bundle = TerminalBundle::new().with_size(size);
    let terminal = &mut term_bundle.terminal;

    commands.spawn_bundle(term_bundle);

    commands.spawn_bundle(TiledCameraBundle::new()
        .with_tile_count(size));

    commands.spawn().insert(Renderable{ tile: Tile {
            glyph: '_',
            fg_color: Color::WHITE,
            bg_color: Color::rgba(1.0, 0.0, 0.0, 1.0),
        },
        order: 64
    })
    .insert(Position{
        x: 0,
        y: 0,
    });
    
    commands.spawn().insert(Renderable{ tile: Tile {
            glyph: '@',
            fg_color: Color::RED,
            bg_color: Color::rgba(1.0, 1.0, 0.0, 0.5),
        },
    order: 128
    })
    .insert(Position{
        x: 0,
        y: 0,
    });

    
}

pub fn update_render_order(mut commands: Commands, mut order: ResMut<RenderOrder>, query: Query<(Entity, &Renderable, &Position)>) {
    //let mut entities = query.iter().collect::<Vec<_>>();
    //entities.sort_by_key(|e| e.1.order);
    commands.insert_resource(query.iter().collect::<Vec<_>>().sort_by_key(|e| e.1.order));
}

/// Rendering system
/// Should render all tiles and all entities with "render"
pub fn render(mut commands: Commands, query: Query<(&Renderable, &Position)>, mut term_query: Query<&mut Terminal>) {
    let mut terminal = term_query.single_mut();
    
    terminal.clear();
    
    //terminal.draw_border_single();

    for (rend, pos) in query.iter() {
        let current_tile = terminal.get_tile([pos.x, pos.y]);

        let tile = rend.tile;
        
        if tile.bg_color.a() == 1.0 {
            terminal.put_tile([pos.x, pos.y], tile);
        }
        else if tile.bg_color.a() == 0.0 {
            let new_tile = Tile {
                glyph: tile.glyph,
                fg_color: tile.fg_color,
                bg_color: current_tile.bg_color,
            };

            terminal.put_tile([pos.x, pos.y], new_tile);
        }
        else {
            let new_tile = Tile {
                glyph: tile.glyph,
                fg_color: tile.fg_color,
                bg_color: blend_colors(tile.bg_color, current_tile.bg_color),
            };
            terminal.put_tile([pos.x, pos.y], new_tile);
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