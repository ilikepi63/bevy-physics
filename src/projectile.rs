use crate::{
    damage::Damage, damage_text::spawn_damage_text_on_entity, health::Health, hit_box::HitBox,
    utils::safe_minus,
};
use bevy::prelude::*;
use std::cmp::min;

#[derive(Component)]
pub struct Projectile {
    pub despawn_after_hit: bool,
    pub direction: Vec3,
    pub speed: f32,
}

pub fn projectile_system(
    mut commands: Commands,
    mut hitboxes: Query<(Entity, &Transform, &HitBox, &mut Health)>,
    projectiles: Query<(Entity, &Transform, &Projectile, &Damage)>,
) {
    for (entity, transform, hitbox, mut health) in hitboxes.iter_mut() {
        for (projectile_entity, projectile_transform, projectile, damage) in projectiles.iter() {
            // calculate that a projectile is in the range
            // let  = transform.translation.z - projectile_transform.translation.z;
            let (hitbox_translation, projectile_translation) =
                (transform.translation, projectile_transform.translation);

            //TODO: height

            let distance = ((safe_minus(projectile_translation.z, hitbox_translation.z)).powi(2)
                + (safe_minus(projectile_translation.x, hitbox_translation.x)).powi(2))
            .sqrt();

            if distance < hitbox.radius as f32 {
                spawn_damage_text_on_entity(&mut commands, entity, damage.amount);

                health.current = health.current - min(damage.amount, health.current);

                if projectile.despawn_after_hit {
                    commands.entity(projectile_entity).despawn();
                }
            }
        }
    }
}

pub fn projectile_movement_system(mut projectiles: Query<(&mut Transform, &Projectile)>) {
    for (mut transform, projectile) in projectiles.iter_mut() {
        transform.translation = transform.translation + projectile.direction * projectile.speed;
    }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (projectile_system, projectile_movement_system));
    }
}
