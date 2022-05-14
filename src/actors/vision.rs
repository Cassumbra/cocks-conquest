use adam_fov_rs::{fov, VisibilityMap};
use crate::{actions::movement::{PointMoveEvent, Collidables}, rendering::RenderOrder};

use super::*;

pub fn setup_vision (
    mut commands: Commands,

    mut ev_movement_event: EventWriter<PointMoveEvent>,

    vision_query: Query<Entity, With<Vision>>,
) {
    for ent in vision_query.iter() {
        ev_movement_event.send(PointMoveEvent{entity: ent, movement: IVec2::new(0, 0)});
    }
    
    commands.insert_resource(NextState(GameState::Playing));
}

// This and the mindmap will need to store more than just an entity or a tile later when we get to allowing players to look around and examine stuff
// Eventually, we should add a "peeking" mechanic for AI and Player to allow them to look and shoot things as if they were doing so from a position next to themselves.
pub fn update_vision (
    mut ev_point_move: EventReader<PointMoveEvent>,

    collidables: Res<Collidables>,

    mut query: Query<(&Position, &mut Vision)>,
    actor_query: Query<&Position, (With<TakesTurns>, With<Collides>)>,
) {
    if ev_point_move.iter().next().is_some() {
        let mut non_actor_collidables = collidables.0.clone();
        for actor_pos in actor_query.iter() {
            non_actor_collidables[actor_pos.0] = None;
        }
        for (ent_pos, mut ent_vis) in query.iter_mut() {
            ent_vis.0.opaque = non_actor_collidables.clone();
            ent_vis.0.visible = Grid::default([ent_vis.0.visible.width(), ent_vis.0.visible.height()]);
            fov::compute(ent_pos.0, 7, &mut ent_vis.0);
        }
    }
}

pub fn update_mind_map (
    mut ev_point_move: EventReader<PointMoveEvent>,

    order: Res<RenderOrder>,

    mut query: Query<(&Position, &mut Vision, &mut MindMap)>,
    visible_query: Query<(&Renderable, &Position)>,
) {
    if ev_point_move.iter().next().is_some() {
        let (_pos, vis, mut mind_map) = query.single_mut();
        for (index, visible) in vis.0.visible.iter().enumerate() {
            if *visible {
                mind_map.seen[index].clear();
            }
        }

        let seen = mind_map.seen.clone();
        for (index, position) in seen.iter_2d() {
            for (entity, tile) in position {
                if let Ok((rend, rend_pos)) = visible_query.get(*entity) {
                    if vis.0.visible[rend_pos.0] {
                        mind_map.seen[index].retain(|x| x.0 != *entity);
                    }
                }
            }
        }

        for e in order.0.iter() {
            if let Ok((rend, rend_pos)) = visible_query.get(*e) {
                if vis.0.visible[rend_pos.0] {
                    mind_map.seen[rend_pos.0].push((*e, rend.tile));
                }
            }
        }
    }
}

// Data
#[derive(Component, Default, Debug, Clone)]
pub struct Map {
    pub visible: Grid<bool>,
    pub opaque: Grid<Option<Entity>>,
}
impl Map {
    pub fn size(&self) -> IVec2 {
        IVec2::new(self.visible.width() as i32, self.visible.height() as i32)
    }
}

impl VisibilityMap for Map {
    fn is_opaque(&self, p: IVec2) -> bool { self.opaque[p].is_some() }
    fn is_in_bounds(&self, p: IVec2) -> bool { p.cmpge(IVec2::ZERO).all() && p.cmplt(self.size()).all() }
    fn set_visible(&mut self, p: IVec2) { self.visible[p] = true; }
    fn dist(&self, a: IVec2, b: IVec2) -> f32 { a.as_vec2().distance(b.as_vec2()) }
}

// Components
#[derive(Component, Default, Debug, Clone)]
pub struct Vision (pub Map);

#[derive(Component, Default, Clone)]
pub struct MindMap {
    pub seen: Grid<Vec<(Entity, Tile)>>,
}