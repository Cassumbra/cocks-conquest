use bevy::{prelude::*};
use bevy_ascii_terminal::{Tile, Terminal};
use inflector::Inflector;
use serde::{Deserialize, Serialize};
use crate::{actors::{vision::{Vision, MindMap}, stats::{StatVisibility, DebugShowStats}, status_effects::{StatusEffectEvent, RemoveStatusEffectEvent, StatusEffects}}, player::targetting::Targetting, actions::ranged::get_line_points, ai::targetting_behavior::Engages};
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
        .init_resource::<LeftSize>()
        .init_resource::<TemporaryTerminal>();
    }
}

//Components
#[derive(Component, Default, Copy, Clone)]
pub struct Renderable {
    pub base_tile: Tile,
    pub effective_tile: Tile,
    pub order: u8,
}
impl Renderable {
    pub fn new(tile: Tile, order: u8) -> Self {
        Self {base_tile: tile, effective_tile: tile, order}
    }
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

pub struct LeftSize {
    pub width: u32,
}
impl Default for LeftSize {
    fn default() -> LeftSize {
        LeftSize {
            width: 20,
        }
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct TemporaryTerminal(pub Terminal);

//Systems
pub fn update_effective_tiles (
    mut ev_status_effect: EventReader<StatusEffectEvent>,
    mut ev_removed_status_effect: EventReader<RemoveStatusEffectEvent>,

    mut renderable_query: Query<(&mut Renderable, Option<&StatusEffects>)>,
) {
    // TODO performance: THIS IS BAD. We are updating all stats on the actor instead of the changed stat. FIX THIS.
    let mut entities = Vec::<Entity>::new();
    
    // TODO performance: for these first two, we only need to push if the status actually has an effect on stats
    for ev in ev_status_effect.iter() {
        entities.push(ev.entity);
    }
    for ev in ev_removed_status_effect.iter() {
        entities.push(ev.entity);
    }

    // TODO performance: We are looping through all of our things to record things that need changes and then looping through them again. This is 2x more costly than it needs to be.
    for entity in entities {
        if let Ok((mut renderable, opt_statuses)) = renderable_query.get_mut(entity) {
            renderable.effective_tile = renderable.base_tile;
            if let Some(statuses) = opt_statuses {
                for status in statuses.iter() {
                    if let Some(modification) = status.tile_modification {
                        if let Some(glyph_mod) = modification.glyph {
                            renderable.effective_tile.glyph = glyph_mod;
                        }
                        if let Some(bg_mod) = modification.bg_color {
                            renderable.effective_tile.bg_color = bg_mod;
                        }
                        if let Some(fg_mod) = modification.fg_color {
                            renderable.effective_tile.fg_color = fg_mod;
                        }
                    }
                }
            }
        }
    }
}

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

    *terminal = temporary_terminal.0.clone();
    
    temporary_terminal.0 = Terminal::with_size([terminal.width(), terminal.height()]);
}

pub fn render_level_view (
    query: Query<(&Renderable, &Position)>,
    player_query: Query<(&Vision, &MindMap), With<Player>>,

    order: Res<RenderOrder>,
    left_size: Res<LeftSize>,
    bottom_size: Res<BottomSize>,
    mut terminal: ResMut<TemporaryTerminal>,
) {
    let (vis, mind_map) = player_query.single();
    
    for (index, position) in mind_map.seen.iter_2d() {
        for (entity, tile) in position {
            let i_pos_x = index.x + left_size.width as i32;
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
                let i_pos_x = pos.0.x + left_size.width as i32;
                let i_pos_y = pos.0.y + bottom_size.height as i32;
                
                let tile = rend.effective_tile;
                
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

    show_stats: Res<DebugShowStats>,
    bottom_size: Res<BottomSize>,
    left_size: Res<LeftSize>,
    log: Res<Log>,
    mut terminal: ResMut<TemporaryTerminal>,
) {
    let (player, stats, opt_name) = player_query.single();

    let mut name = player.id().to_string();

    if let Some(temp_name) = opt_name {
        name = temp_name.to_string();
    }
    
    let mut print_fragments = Log::fragment_string(format![" {}    ", &name], Color::WHITE);

    for (stat_type, stat) in stats.0.iter() {
        if **show_stats || matches!(stat.visibility, StatVisibility::Public | StatVisibility::Private) {
            print_fragments.append(&mut Log::fragment_string(format!["{}: {}  ", stat_type.to_string().to_title_case(), stat.effective], stat_type.color()));
        }

        
    }
    let [mut current_length, mut current_line] = put_string_vec_formatted([(left_size.width-1) as i32, (bottom_size.height-1) as i32], &print_fragments, &mut terminal.0, EolAction::None);

    // Log rendering
    let lines: &[Vec<LogFragment>];

    if log.lines.len() < bottom_size.height as usize {
        lines = &log.lines[..];
    } else {
        lines = &log.lines[log.lines.len()-bottom_size.height as usize..log.lines.len()]
    }

    for line in lines.iter().rev() {
        current_line -= 1;
        [current_length, current_line] = put_string_vec_formatted([(left_size.width-1) as i32, current_line], line, &mut terminal.0, EolAction::None);
    }
}

pub fn render_actor_info (
    player_query: Query<(Entity, &Vision), With<Player>>,
    actor_query: Query<(Entity, &Position, Option<&Name>, Option<&Stats>, Option<&Engages>), (With<TakesTurns>)>,
    name_query: Query<&Name>,

    show_stats: Res<DebugShowStats>,
    left_size: Res<LeftSize>,
    log: Res<Log>,
    mut terminal: ResMut<TemporaryTerminal>,
) {
    let (player, vision) = player_query.single();

    let mut rng = rand::thread_rng();

    let size = terminal.size();

    let mut print_fragments = Vec::<LogFragment>::new();

    'actor_check: for (actor, pos, opt_name, opt_stats, opt_engages) in actor_query.iter() {
        // Check if actor is visible
        if !vision.visible(**pos) || player == actor {
            continue 'actor_check;
        }

        let mut name = actor.id().to_string();

        if let Some(temp_name) = opt_name {
            name = temp_name.to_string();
        }

        print_fragments.append(&mut Log::fragment_string(format!["{} \n ", name], Color::WHITE));
        
        if let Some(stats) = opt_stats {
            for (stat_type, stat) in stats.0.iter() {
                if **show_stats || matches!(stat.visibility, StatVisibility::Public) {
                    print_fragments.push(LogFragment::new(format!["{}: {}  ", stat_type.abbreviate().to_ascii_uppercase(), stat.effective], stat_type.color()));
                }
                
            }

            print_fragments.append(&mut Log::fragment_string(format!["\n "], Color::WHITE));
        }

        if let Some(engages) = opt_engages {
            if let Some(target) = engages.target {
                let mut target_name = target.id().to_string();

                if let Ok(temp_name) = name_query.get(target) {
                    target_name = temp_name.to_string();
                }

                if engages.get_alert() {
                    print_fragments.push(LogFragment::new(format!["Alert: {}  ", target_name], Color::WHITE));
                } else {
                    print_fragments.push(LogFragment::new(format!["Target: {}  ", target_name], Color::WHITE));
                }
            } else {
                print_fragments.push(LogFragment::new(format!["No target  ", ], Color::WHITE));
            }

            
        }

        print_fragments.append(&mut Log::fragment_string(format!["\n \n "], Color::WHITE));


        //terminal.0.put_string([0, (size.y - 1 - i as u32 ) as i32], &String::from("AU"));
    }

    let [mut current_length, mut current_line] = put_string_vec_formatted([0, (size.y - 1) as i32], &print_fragments, &mut terminal.0, EolAction::Wrap(left_size.width as i32));
}

pub fn render_targetting (
    left_size: Res<LeftSize>,
    bottom_size: Res<BottomSize>,
    targetting: Res<Targetting>,
    mut terminal: ResMut<TemporaryTerminal>,
) {
    // TODO: draw line
    let distance = targetting.position.as_vec2().distance(targetting.target.as_vec2());

    let mut points = get_line_points(targetting.position.as_vec2(), targetting.target.as_vec2(), distance);

    points.pop_front();
    //points.push_back(targetting.target);

    for (i, point) in points.iter().enumerate() {
        let i_pos_x = point.x + left_size.width as i32;
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

pub fn render_help_page (

) {
    
}

// Helper Systems
fn put_string_vec (
    position: [i32; 2],
    strings: &Vec<String>,
    terminal: &mut Terminal,
) -> [i32; 2] {
    let [mut current_length, mut current_line] = position;
    for string in strings.iter() {

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
        
        terminal.put_string([current_length, current_line], string);
        current_length += string.len() as i32;
    }
    [current_length, current_line]
}

// TODO: Add thing for limiting where text is placed (how far to the right it can go, also maybe how far down it can go?) other than terminal limits
//       Could be an enum type thing. Could give option to abbreviate instead of wrap around (remove vowels until enough space. remove inner consonants if no more vowels to remove)
//       Enum could also have option to remove last three characters and replace them with ...
pub fn put_string_vec_formatted (
    position: [i32; 2],
    fragments: &Vec<LogFragment>,
    terminal: &mut Terminal,
    eol_action: EolAction,
) -> [i32; 2] {
    let [mut current_length, mut current_line] = position;
    for fragment in fragments.iter() {
        let string = &fragment.text;

        if string == "\n " {
            
            current_length = position[0];
            current_line -= 1;
            continue;
        }
        else if string == "" {
            continue;
        }

        match eol_action {
            EolAction::Wrap(width) => {
                if current_length + string.len() as i32 > width {
                    current_length = position[0];
                    current_line -= 1;
                    if !terminal.is_in_bounds([current_length + string.len() as i32, current_line]) {
                        //eprintln!("ERROR: Cannot fit strings in terminal!");
                        break;
                    }
                }
            },
            EolAction::None => {},
        }

        if !terminal.is_in_bounds([current_length - 1 + string.len() as i32, current_line]) {
            current_length = position[0];
            current_line -= 1;
            if !terminal.is_in_bounds([current_length - 1 + string.len() as i32, current_line]) {
                //eprintln!("ERROR: Cannot fit strings in terminal!");
                break;
            }
        }
        
        terminal.put_string([current_length, current_line], string.fg(fragment.color));
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

// Data
pub enum EolAction {
    //Abbreviate,
    //Truncate,
    Wrap(i32),
    None,
}