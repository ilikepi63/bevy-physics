// use bevy_healu
use bevy::{prelude::*};

#[derive(Component)]
pub struct Health {
    pub max: u32,
    pub current: u32,
}

pub fn health_system(mut commands: Commands, entities: Query<(Entity, &Health)>) {
    for (entity, health) in entities.iter() {
        if health.current == 0 {
            commands.entity(entity).despawn();
        }
    }
}
