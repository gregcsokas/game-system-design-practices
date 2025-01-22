pub mod damage;
pub mod health;


#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use crate::damage::{component::Damage, event::HitEvent, system::deal_damage};
    use crate::health::{component::{Health, Dead}, system::death_check_system};

    fn setup_test_app() -> App {
        let mut app = App::new();
        
        app.add_plugins(MinimalPlugins)
            .add_event::<HitEvent>()
            .add_systems(Update, (deal_damage, death_check_system).chain());
        
        app
    }
    
    #[test]
    fn test_event_based_damage_system() {
        let mut app = setup_test_app();
        
        let test_entity = app.world_mut().spawn(
            Health::default()
        ).id();
        
        let damage_10 = Damage::new(10.0);
        
        app.world_mut().send_event(HitEvent { target: test_entity, damage: damage_10 });
        
        app.update();
    
        let health = app.world().entity(test_entity).get::<Health>().unwrap();
        
        assert_eq!(health.get_health(), 90.0);
    }
    
    #[test]
    fn test_hit_entity_until_dead() {
        let mut app = setup_test_app();
        
        let test_entity = app.world_mut().spawn(
            Health::default()
        ).id();
        
        let damage_10 = Damage::new(10.0);
        
        for _ in 0..9 {
            app.world_mut().send_event(HitEvent { target: test_entity, damage: damage_10 });
            app.update();
        }
        
        let health = app.world().entity(test_entity).get::<Health>().unwrap();
        
        assert_eq!(health.get_health(), 10.0);
        
        app.world_mut().send_event(HitEvent { target: test_entity, damage: damage_10 });
        app.update();
        
        let health = app.world().entity(test_entity).get::<Health>().unwrap();
        
        assert_eq!(health.get_health(), 0.0);
        
        assert!(app.world().entity(test_entity).contains::<Dead>());
    }
}
