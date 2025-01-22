use bevy::prelude::*;
use crate::damage::component::Damage;

#[derive(Event)]
pub struct HitEvent {
    pub target: Entity,
    pub damage: Damage
}