use std::cmp::{min, max};
use bevy::prelude::*;
use sark_grids::grid::Grid;
use rand::Rng;
use super::*;


// Bundles
#[derive(Bundle, Copy, Clone)]
pub struct WallBundle {
    pub position: Position,
    pub renderable: Renderable,
    pub collides: Collides,
}
impl Default for WallBundle {
    fn default() -> WallBundle {
        WallBundle {
            position: Position (IVec2::new(0, 0)),
            renderable: Renderable {
                tile: Tile {
                    glyph: '#',
                    fg_color: Color::WHITE,
                    bg_color: Color::BLACK,
                },
                order: 32
            },
            collides: Collides,
        }
    }
}

#[derive(Bundle, Copy, Clone)]
pub struct FloorBundle {
    pub position: Position,
    pub renderable: Renderable,
}
impl Default for FloorBundle {
    fn default() -> FloorBundle {
        FloorBundle {
            position: Position (IVec2::new(0, 0)),
            renderable: Renderable {
                tile: Tile {
                    glyph: '.',
                    fg_color: Color::DARK_GRAY,
                    bg_color: Color::BLACK,
                },
                order: 48
            },
        }
    }
}

//Systems
pub fn entity_map_rooms_passages (
    mut has_run: Local<bool>,
    mut commands: Commands,
    map_size: Res<MapSize>,
) {
    

    let mut map_objects: Grid<Option<Entity>> = Grid::default([map_size.width, map_size.height]);

    let mut rng = rand::thread_rng();

    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let wall_rect = Rectangle::new(IVec2::new(0, 0), map_size.width as i32 - 1, map_size.height as i32 - 1);
    fill_rect(&mut commands, &mut map_objects, WallBundle{..Default::default()}, &wall_rect);

    let mut rooms = Vec::<Rectangle>::new();

    for _i in 0..=MAX_ROOMS {
        let w = rng.gen_range(MIN_SIZE..MAX_SIZE);
        let h = rng.gen_range(MIN_SIZE..MAX_SIZE);
        let x = rng.gen_range(1..(map_size.width as i32 - w - 1));
        let y = rng.gen_range(1..(map_size.height as i32 - h - 1));

        

        let room = Rectangle::new(IVec2::new(x, y), w, h);
        //let room_ent = commands.spawn().insert(rect).id();

        let mut ok = true;
        for other_room in rooms.iter() {
            if room.intersect(other_room) { ok = false }
        }
        if ok {
            fill_rect (&mut commands, &mut map_objects, FloorBundle{..Default::default()}, &room);

            if !rooms.is_empty() {
                let center = room.center();
                let previous_center = rooms[rooms.len()-1].center();
                if rng.gen_range(0..=1) == 1 {
                    fill_row(&mut commands, &mut map_objects, FloorBundle{..Default::default()}, previous_center.x, center.x, previous_center.y);
                    fill_column(&mut commands, &mut map_objects, FloorBundle{..Default::default()}, previous_center.y, center.y, center.x);
                } else {
                    fill_column(&mut commands, &mut map_objects, FloorBundle{..Default::default()}, previous_center.y, center.y, previous_center.x);
                    fill_row(&mut commands, &mut map_objects, FloorBundle{..Default::default()}, previous_center.x, center.x, center.y);
                }
            }

            rooms.push(room);
            commands.spawn().insert(room);
        }
    }

    commands.insert_resource(Rooms(rooms));
    *has_run = true;    
}

fn simple_entity_map(
    mut commands: Commands,
    map_size: Res<MapSize>,
    collidables: Res<Collidables>,
) {
    let mut rng = rand::thread_rng();

    draw_line_cardinal(&mut commands, IVec2::new(0, 0), IVec2::new(0, map_size.height as i32 - 1));
    draw_line_cardinal(&mut commands, IVec2::new(map_size.width as i32 - 1, 0), IVec2::new(map_size.width as i32 - 1, map_size.height as i32 - 1));

    draw_line_cardinal(&mut commands, IVec2::new(0, map_size.height as i32 - 1), IVec2::new(map_size.width as i32 - 1, map_size.height as i32 - 1));
    draw_line_cardinal(&mut commands, IVec2::new(0, 0), IVec2::new(map_size.width as i32 - 1, 0));

    for _i in 0..100 {
        let x = rng.gen_range(0..map_size.width);
        let y = rng.gen_range(0..map_size.height);

        commands.spawn_bundle(WallBundle{
            position: Position (IVec2::new(x as i32, y as i32)),
            ..Default::default()
        });
    }
}

fn draw_line_cardinal( commands: &mut Commands, pos1: IVec2, pos2: IVec2 ) {
    if pos1.x == pos2.x {
        for i in pos1.y ..= pos2.y {
            commands.spawn_bundle(WallBundle{
                position: Position (IVec2::new(pos1.x, i)),
                ..Default::default()
            });
        }
    }
    else if pos1.y == pos2.y {
        for i in pos1.x ..= pos2.x {
            commands.spawn_bundle(WallBundle{
                position: Position (IVec2::new(i, pos1.y)),
                ..Default::default()
            });
        }
    }
    else {
        eprintln!("ERROR: Not a cardinal direction!");
    }
}

// Functions
fn fill_rect ( commands: &mut Commands, map_objects: &mut Grid<Option<Entity>>, bundle: impl Bundle + Copy, rect: &Rectangle) {
    
    
    for pos in map_objects.clone().rect_iter([rect.pos1.x, rect.pos1.y]..=[rect.pos2.x, rect.pos2.y]) {
        if pos.1.is_some() {
            let old_entity = map_objects[[pos.0.x as u32, pos.0.y as u32]].unwrap();
            commands.entity(old_entity).despawn();
        }
        let entity = commands.spawn_bundle(bundle)
            .insert(Position(IVec2::new(pos.0.x, pos.0.y)))
            .id();
        
        map_objects[[pos.0.x as u32, pos.0.y as u32]] = Some(entity);
    }
}

fn fill_row(commands: &mut Commands, map_objects: &mut Grid<Option<Entity>>, bundle: impl Bundle + Copy, x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        if map_objects[[x as u32, y as u32]].is_some() {
            let old_entity = map_objects[[x as u32, y as u32]].unwrap();
            commands.entity(old_entity).despawn();
        }
        let entity = commands.spawn_bundle(bundle)
            .insert(Position(IVec2::new(x, y)))
            .id();
        
        map_objects[[x as u32, y as u32]] = Some(entity);
    }
}

fn fill_column(commands: &mut Commands, map_objects: &mut Grid<Option<Entity>>, bundle: impl Bundle + Copy, y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        if map_objects[[x as u32, y as u32]].is_some() {
            let old_entity = map_objects[[x as u32, y as u32]].unwrap();
            commands.entity(old_entity).despawn();
        }
        let entity = commands.spawn_bundle(bundle)
            .insert(Position(IVec2::new(x, y)))
            .id();
        
        map_objects[[x as u32, y as u32]] = Some(entity);
    }
}



