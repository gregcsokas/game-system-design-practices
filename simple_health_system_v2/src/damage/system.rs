use bevy::prelude::*;
use crate::damage::event::HitEvent;
use crate::health::component::{Dead, Health};

pub(crate) fn deal_damage(
    _commands: Commands,
    mut query: Query<(Entity, &mut Health), Without<Dead>>,
    mut hit_event_reader: EventReader<HitEvent>
) {
    for hit in hit_event_reader.read() {
        if let Ok((entity, mut health)) = query.get_mut(hit.target) {
            health.take_damage(hit.damage.get_damage());
            println!("Entity {:?} took {} damage", hit.target, hit.damage.get_damage());
        }
    }
}