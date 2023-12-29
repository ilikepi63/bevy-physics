use std::cmp::min;

use bevy::{prelude::*, utils::Uuid};
use bevy_rapier3d::prelude::*;

use crate::{
    auras::{apply_overtime, OvertimeComponent, Overtime},
    character_controller::Player,
    damage::Damage,
    damage_text::spawn_damage_text_on_entity,
    health::Health,
    lifetime::Lifetime,
    projectile::Projectile,
    utils,
};

use super::model::CastSpellFire;

static SPELL_UUID: &str = "s";
static MELEE_UUID: &str = "a";

fn spell(
    mut cast_spell_fire_events: EventReader<CastSpellFire>,
    mut character_query: Query<&mut Transform, With<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    // This is a big query right now
    mut other_entities: Query<
        (
            Entity,
            &Transform,
            &mut Health,
            Option<&mut OvertimeComponent>,
        ),
        (With<Health>, Without<Player>),
    >,
) {
    for event in &mut cast_spell_fire_events {
        match event.id.as_str() {
            "s" => {
                let mut character = character_query.single_mut();
                cast_spell(&mut character, &mut commands, &mut meshes, &mut materials)
            }
            "a" => {}
            _ => {}
        }
    }
}

fn cast_spell(
    character: &mut Transform,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let direction = character.forward();

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.2,
                stacks: 18,
                sectors: 36,
            })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_translation(character.translation),
            ..default()
        })
        .insert(Lifetime {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        })
        .insert(Velocity::linear(direction * 100.0))
        .insert(Name::new("Bullet"))
        .insert(RigidBody::Dynamic)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ContactForceEventThreshold(30.0))
        .insert(Collider::ball(1.0))
        .insert(Damage { amount: 10 })
        .insert(Projectile {
            despawn_after_hit: true,
        });
}

fn basic_attack(
    // buttons: Res<Input<KeyCode>>,
    player: &Transform,
    mut commands: Commands,
    mut other_entities: Query<
        (
            Entity,
            &Transform,
            &mut Health,
            Option<&mut OvertimeComponent>,
        ),
        (With<Health>, Without<Player>),
    >,
) {
    let player = player.translation;

    for (entity, transform, mut health, mut overtime_comp) in other_entities.iter_mut() {
        // check if the entity is within radius
        let distance = ((utils::safe_minus(transform.translation.z, player.z)).powi(2)
            + (utils::safe_minus(transform.translation.x, player.x)).powi(2))
        .sqrt();

        // this just takes into account distance along the xz plane
        if distance < 2.0 {
            let amount = 10;

            spawn_damage_text_on_entity(&mut commands, entity, amount);

            health.current = health.current - min(amount, health.current);

            apply_overtime(
                entity,
                &mut commands,
                Overtime::damage_per_second(3, 5),
                &mut overtime_comp,
            );
        }
    }

    // calculate front and sides
}
