use bevy::prelude::*;

use crate::{actors::stats::{Stats, StatType, StatChangeEvent}, log::Log};




// Events
pub struct HealActionEvent{
    pub healing_entity: Entity,
}

// Systems
pub fn heal_action (
    mut heal_query: Query<(Entity, &mut Stats, Option<&Name>), With<CanHeal>>,

    mut ev_heal_event: EventReader<HealActionEvent>,
    mut ev_stat_change: EventWriter<StatChangeEvent>,

    mut log: ResMut<Log>,
) {
    for ev in ev_heal_event.iter() {
        if let Ok((entity, mut stats, opt_name)) = heal_query.get_mut(ev.healing_entity) {
            let name = if opt_name.is_some() {opt_name.unwrap().to_string()} else {ev.healing_entity.id().to_string()};

            // TODO: Both of these should be retrieved dynamically from the CanHeal component and/or a MaxStats component in the future.
            if stats.get_effective(&StatType::CumPoints) >= 5 && stats.get_effective(&StatType::Health) < stats.get_max(&StatType::Health) {
                log.log_string_formatted(format!(" {} uses 5 cum points to heal for 1 health.", name), Color::GREEN);
                ev_stat_change.send(StatChangeEvent::new(StatType::CumPoints, -5, entity));
                ev_stat_change.send(StatChangeEvent::new(StatType::Health, 1, entity));
            }
        }
    }

}


// Components
// Should be dynamic like how attacks are (at some point but i dont care)
#[derive(Component, Default, Copy, Clone)]
pub struct CanHeal;

