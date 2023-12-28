use std::cmp::min;

use bevy::prelude::*;

use crate::{damage_text::spawn_damage_text_on_entity, health::Health};

#[derive(Component)]
pub struct Damage {
    pub amount: u32,
}

pub fn apply_damage(mut commands: &mut Commands, entity: Entity, amount: u32, health: &mut Health) {
    spawn_damage_text_on_entity(&mut commands, entity, amount);

    health.current = health.current - min(amount, health.current);
}

pub fn apply_health(mut commands: &mut Commands, entity: Entity, amount: u32, health: &mut Health) {
    spawn_damage_text_on_entity(&mut commands, entity, amount);

    if health.current < health.max {
        health.current = min(health.current + amount, health.max);
    }
}
