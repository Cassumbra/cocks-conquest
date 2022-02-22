// Unglob later
use bevy::prelude::*;
use bevy::render::color::*;
use bevy_ascii_terminal::*;
use bevy_tiled_camera::*;
use super::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlasses: ResMut<Assets<TextureAtlas>>) {
    let font_handle = asset_server.load("CGA8x8thick_transparent.png");
    let font_atlas = TextureAtlas::from_grid(font_handle, Vec2::new(8.0, 8.0), 16, 16);
    let font_atlas_handle = texture_atlasses.add(font_atlas);

    let size = [20, 20];

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //commands.spawn_bundle(TiledCameraBundle::new()
    //    .with_tile_count(size)
    //    .with_pixels_per_tile(8));

    commands.spawn_bundle(SpriteSheetBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        visibility: Visibility { is_visible: true },
        texture_atlas: font_atlas_handle.clone(),
        sprite: TextureAtlasSprite{
            color: Color::BLUE,
            bg_color: Color::BLACK,
            index: 1,
            flip_x: false,
            flip_y: false,
            custom_size: None,
        },
        ..Default::default()
    });

    commands.spawn_bundle(SpriteSheetBundle {
        transform: Transform::from_xyz(1.0, 0.0, 0.0),
        visibility: Visibility { is_visible: true },
        texture_atlas: font_atlas_handle.clone(),
        sprite: TextureAtlasSprite{
            color: Color::BLUE,
            bg_color: Color::BLACK,
            index: 1,
            flip_x: false,
            flip_y: false,
            custom_size: None,
        },
        ..Default::default()
    });

    commands.spawn_bundle(SpriteSheetBundle {
        transform: Transform::from_xyz(2.0, 0.0, 0.0),
        visibility: Visibility { is_visible: true },
        texture_atlas: font_atlas_handle.clone(),
        sprite: TextureAtlasSprite{
            color: Color::BLUE,
            bg_color: Color::BLACK,
            index: 1,
            flip_x: false,
            flip_y: false,
            custom_size: None,
        },
        ..Default::default()
    });
    
    commands.spawn_bundle(SpriteSheetBundle {
        transform: Transform::from_xyz(24.0, 0.0, 0.0),
        visibility: Visibility { is_visible: true },
        texture_atlas: font_atlas_handle.clone(),
        sprite: TextureAtlasSprite{
            color: Color::BLUE,
            bg_color: Color::BLACK,
            index: 1,
            flip_x: false,
            flip_y: false,
            custom_size: None,
        },
        ..Default::default()
    });

}

pub fn update_render_order(mut commands: Commands, mut order: ResMut<RenderOrder>, query: Query<(Entity, &Renderable, &Position)>) {
    //let mut entities = query.iter().collect::<Vec<_>>();
    //entities.sort_by_key(|e| e.1.order);
    commands.insert_resource(query.iter().collect::<Vec<_>>().sort_by_key(|e| e.1.order));
}
/*
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
}*/