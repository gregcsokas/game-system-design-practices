use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Damage {
    damage: f32,
}

impl Default for Damage {
    fn default() -> Self {
        Damage::new(10.0)
    }
}

impl Damage {
    pub fn new(damage: f32) -> Self {
        Self { damage }
    }
    
    pub fn get_damage(&self) -> f32 {
        self.damage
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_damage_component() {
        let damage = Damage::new(10.0);
        
        assert_eq!(damage.get_damage(), 10.0);
    }
}