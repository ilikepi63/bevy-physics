
use bevy::prelude::*;
use crate::{hit_box::HitBox, health::Health, bullet::Damage};
use std::cmp::min;

#[derive(Component)]
pub struct Projectile{
    pub despawn_after_hit: bool
}

pub fn projectile_system(
    mut commands: Commands,
    mut hitboxes: Query<(&Transform, &HitBox, &mut Health)>,
    projectiles: Query<(Entity, &Transform, &Projectile, &Damage)>
) {

    for (transform, hitbox, mut health) in hitboxes.iter_mut() {
        for (projectile_entity, projectile_transform, projectile, damage) in projectiles.iter() {


            // calculate that a projectile is in the range
            // let  = transform.translation.z - projectile_transform.translation.z;
            let (hitbox_translation, projectile_translation) = (transform.translation, projectile_transform.translation);

                //TODO: height 

            let distance = ( (safe_minus(projectile_translation.z, hitbox_translation.z) ).powi(2) + (safe_minus(projectile_translation.x, hitbox_translation.x)).powi(2) ).sqrt();

            info!("Distance: {}", distance);

            if distance < hitbox.radius as f32 {

                health.current = health.current - min(damage.amount, health.current);

                if projectile.despawn_after_hit {
                    commands.entity(projectile_entity).despawn();
                }

            }

        }
    }

} 

pub fn safe_minus(one: f32, two: f32) -> f32 {
    if one >= two {
        one - two
    }else{
        two - one
    }
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin{
    fn build(&self, app: &mut App) {
        app.add_system(projectile_system);
    }
}