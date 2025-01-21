use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Dead;


#[derive(Component, PartialOrd, PartialEq)]
pub(crate) struct Health {
    max_health: f32,
    current_health: f32,
}

impl Default for Health {
    fn default() -> Self {
        Health::new(100.0, 100.0)
    }
}

impl Health {
    pub fn new(max_health: f32, current_health: f32) -> Self {
        Self {
            max_health,
            current_health,
        }
    }
    
    pub fn take_damage(&mut self, damage: f32) {
        self.current_health = (self.current_health - damage).max(0.0);
    }
    
    pub fn heal(&mut self, heal: f32) {
        self.current_health = (self.current_health + heal).min(self.max_health);
    }
    
    pub fn get_health(&self) -> f32 {
        self.current_health
    }
    
    pub fn is_dead(&self) -> bool {
        self.current_health <= 0.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_component() {
        let health = Health::default();
        
        assert_eq!(health.current_health, 100.0);
        assert_eq!(health.max_health, 100.0);
    }
    
    #[test]
    fn test_take_damage() {
        let mut health = Health::default();
        health.take_damage(10.0);
        
        assert_eq!(health.current_health, 90.0);
    }
    
    #[test]
    fn test_take_damage_when_dead() {
        let mut health = Health::default();
        health.take_damage(100.0);
        
        assert_eq!(health.current_health, 0.0);
        
        health.take_damage(100.0);
        assert_eq!(health.current_health, 0.0);
    }
    
}
    