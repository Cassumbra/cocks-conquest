use bevy::prelude::*;

use crate::{actors::stats::{Stats, StatType}, log::Log};




// Events
pub struct HealActionEvent{
    pub healing_entity: Entity,
}

// Systems
pub fn heal_action (
    mut heal_query: Query<(&mut Stats, Option<&Name>), With<CanHeal>>,

    mut ev_heal_event: EventReader<HealActionEvent>,

    mut log: ResMut<Log>,
) {
    for ev in ev_heal_event.iter() {
        if let Ok((mut stats, opt_name)) = heal_query.get_mut(ev.healing_entity) {
            let name = if opt_name.is_some() {opt_name.unwrap().to_string()} else {ev.healing_entity.id().to_string()};

            // Both of these should be retrieved dynamically from the CanHeal component and/or a MaxStats component in the future.
            if stats.get_value(&StatType::Health) >= 5 && stats.get_value(&StatType::Health) < stats.get_max(&StatType::Health) {
                log.log_string_formatted(format!(" {} uses 5 cum points to heal for 1 health.", name), Color::GREEN);
                stats.0.get_mut(&StatType::CumPoints).unwrap().value -= 5;
                stats.0.get_mut(&StatType::Health).unwrap().value += 1;
            }
        }
    }

}

// Components
// Should be dynamic like how attacks are (at some point but i dont care)
#[derive(Component, Default, Copy, Clone)]
pub struct CanHeal;