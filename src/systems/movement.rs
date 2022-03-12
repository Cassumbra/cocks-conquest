// Unglob later
use bevy::prelude::*;
use sark_grids::grid::Grid;
use super::super::*;

pub fn do_point_move(
    mut commands: Commands,
    mut ev_collidable_change: EventWriter<CollidableChangeEvent>,
    mut ev_movement_event: EventReader<PointMoveEvent>,
    mut query: Query<(&mut Position, Option<&Collides>, Option<&Name>)>,
    map_size: Res<MapSize>,
    collidables: Res<Collidables>,
) {
    for ev in ev_movement_event.iter() {
        if let Ok((mut pos, col, opt_name)) = query.get_mut(ev.entity) {
            let new_pos = pos.0 + ev.movement;

            if new_pos.x >= map_size.width as i32 || new_pos.y >= map_size.height as i32 || new_pos.x <= -1 || new_pos.y <= -1 ||
                col.is_some() && collidables.0[[new_pos.x as u32, new_pos.y as u32 ]].is_some() {
                
                // When we remake our game, we should create some logs.
                // We need a log that is shown to the player, and we need a debug log.
                // An error log may be good too.
                // Error and debug may be combined?
                if let Some(name) = opt_name {
                    println!("{} bonked at {:?}.", name.as_str(), pos.0)
                }
                else {
                    println!("{:?} bonked at {:?}.", ev.entity, pos.0)
                }
            }
            else {
                pos.0 = new_pos;
            }

            ev_collidable_change.send(CollidableChangeEvent{
                old_position: pos.0,
                new_position: new_pos,
                entity: ev.entity,
            });
        }
    }
}

pub fn update_collidables_new( 
    mut commands: Commands,
    mut ev_collidable_change: EventReader<CollidableChangeEvent>,
    query: Query<(Entity, &Position), (With<Collides>, Added<Collides>)>,
    mut collidables: ResMut<Collidables>,
) {
    let mut collidable_grid = collidables.0.clone();

    for (ent, pos) in query.iter() {
        println!("SHOULD ONLY SEE THIS A FEW TIMES");
        collidable_grid[[pos.0.x as u32, pos.0.y as u32]] = Some(ent);
    }
    for ev in ev_collidable_change.iter() {
        
        collidable_grid[[ev.old_position.x as u32, ev.old_position.y as u32]] = None;
        collidable_grid[[ev.new_position.x as u32, ev.new_position.y as u32]] = Some(ev.entity);
    }

    commands.insert_resource(Collidables(collidable_grid));
}

pub fn update_collidables( 
    mut commands: Commands,
    query: Query<(Entity, &Position), With<Collides>>,
    collidables_changed: Query<(&Collides, &Position), Or<(Changed<Collides>, Added<Collides>, Changed<Position>, Added<Position>)>>,
    map_size: Res<MapSize>,
) {
    if collidables_changed.iter().next().is_some() {
        let mut collidables: Grid<Option<Entity>> = Grid::default([map_size.width, map_size.height]);

        for (ent, pos) in query.iter() {
            if collidables[[pos.0.x as u32, pos.0.y as u32]].is_some() {
                eprintln!("ERROR: Collidables clipping! Destroying old collidable!");
                let old_ent = collidables[[pos.0.x as u32, pos.0.y as u32]].unwrap();
                commands.entity(old_ent).despawn();
            }
            collidables[[pos.0.x as u32, pos.0.y as u32]] = Some(ent);
        }

        commands.insert_resource(Collidables(collidables));

    }
}