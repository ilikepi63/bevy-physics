use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

fn lifetime_despawn(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {

    // info!("Running Lifetime");

    for (entity, mut lifetime) in &mut entities {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct LifetimePlugin;

impl Plugin for LifetimePlugin{
    fn build(&self, app: &mut App) {
        app.add_system(lifetime_despawn);
    }
}