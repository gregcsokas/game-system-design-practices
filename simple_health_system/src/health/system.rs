use bevy::prelude::*;

use crate::health::component::{Health, Dead};


fn deal_damage(
    _commands: Commands,
    mut query: Query<(Entity, &mut Health), Without<Dead>>
) {
    for (entity, mut entity_w_health) in query.iter_mut() {
        let damage = 10.0;
        entity_w_health.take_damage(damage);
        
        println!("Entity {} took {} damage", entity, damage);
    }
}


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
fn death_check_system(
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

#[cfg(test)]
mod tests {
    use super::*;
    
    fn setup_test_app() -> App {
        let mut app = App::new();
        
        app.add_plugins(MinimalPlugins)
            .add_systems(Update, (deal_damage, death_check_system).chain());
        
        app
    }
    
    #[test]
    fn test_damage_system() {
        let mut app = setup_test_app();
        
        let test_entity = app.world_mut().spawn(
            Health::default()
        ).id();
        
        app.update();
        
        let health = app.world().entity(test_entity).get::<Health>().unwrap();
        
        assert_eq!(health.get_health(), 90.0);
    }
    
    #[test]
    fn test_death_system() {
        let mut app = setup_test_app();
        
        let test_entity = app.world_mut().spawn(
            Health::new(100.0, 5.0)
        ).id();
        
        app.update();

        assert!(app.world().entity(test_entity).contains::<Dead>());
    }
    
    #[test]
    fn test_dead_entity_does_not_take_damage() {
        let mut app = setup_test_app();
        
        let test_entity = app.world_mut().spawn(
            Health::new(100.0, 0.0)
        ).id();
        
        app.update();
        
        let health = app.world().entity(test_entity).get::<Health>().unwrap();
        
        assert_eq!(health.get_health(), 0.0);
    }
    
    #[test]
    fn test_healing_system() {
        let mut app = setup_test_app();
        
        let test_entity = app.world_mut().spawn(
            Health::new(100.0, 20.0)
        ).id();
        
        app.add_systems(Update, healing_system);
        
        app.update();
        
        let health = app.world().entity(test_entity).get::<Health>().unwrap();
        
        assert_eq!(health.get_health(), 30.0);
    }
}