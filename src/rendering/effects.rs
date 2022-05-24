// We'll put graphical effects here.
// ie: the animation of projectiles
// Handling the graphical effect separately means we can continue gameplay without holding up the game with animations.

// TODO: Ranged attack effects

use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_ascii_terminal::Tile;

use super::{TemporaryTerminal, BottomSize};


// Systems
pub fn render_effects (
    mut effect_query: Query<(Entity, &mut Effect)>,

    mut commands: Commands,

    time: Res<Time>,
    bottom_size: Res<BottomSize>,
    mut terminal: ResMut<TemporaryTerminal>,
) {
    for (effect_ent, mut effect) in effect_query.iter_mut() {
        effect.current_time += time.delta();

        if let Some(fragment) = effect.fragment_from_time(effect.current_time) {
            for effect_tile in &fragment.tiles {
                terminal.0.put_tile([effect_tile.position.x, effect_tile.position.y + bottom_size.height as i32], effect_tile.tile)
            }
        }
        else {
            commands.entity(effect_ent).despawn();
        }
    }
}


// Data
pub struct EffectTile {
    pub position: IVec2,
    pub tile: Tile,
}

pub struct EffectFragment {
    pub duration: Duration,
    pub tiles: Vec<EffectTile>,
}

#[derive(Component, Default)]
pub struct Effect {
    pub current_time: Duration,
    pub fragments: Vec<EffectFragment>,
}
impl Effect {
    pub fn fragment_from_time(&self, time: Duration) -> Option<&EffectFragment> {
        let mut fragment_time = Duration::from_secs_f32(0.0);

        for fragment in &self.fragments {
            fragment_time += fragment.duration;

            if time <= fragment_time {
                return Some(fragment);
            } 
        }

        None
    }
}