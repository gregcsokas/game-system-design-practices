use bevy::prelude::*;
use crate::health::component::{Dead, Health};


fn healing_system(
    _commands: Commands,
    mut query: Query<(Entity, &mut Health), Without<Dead>>
) {
    for (entity, mut entity_w_health) in query.iter_mut() {
        let heal = 20.0;
        entity_w_health.heal(heal);
        
        println!("Entity {} healed {} health", entity, heal);
    }
}


pub(crate) fn death_check_system(
    mut commands: Commands,
    query: Query<(Entity, &Health), Without<Dead>>
) {
    for (entity, entity_w_health) in query.iter() {
        if entity_w_health.is_dead() {
            
            println!("Entity {} is dead", entity);
            
            commands.entity(entity).insert(Dead);
        }
    }
}