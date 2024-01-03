use std::{cmp::min, time::Duration};

use bevy::{prelude::*, utils::Uuid};
use bevy_hanabi::{EffectAsset, ParticleEffect, ParticleEffectBundle};
use bevy_rapier3d::prelude::*;

use crate::{
    auras::{apply_overtime, OvertimeComponent, Overtime},
    character_controller::Player,
    damage::Damage,
    damage_text::spawn_damage_text_on_entity,
    health::Health,
    lifetime::Lifetime,
    projectile::Projectile,
    utils, particles::make_particle,
};

use super::{model::CastSpellFire, CastSpellInit, CastTime, casting::Casting};

static SPELL_UUID: &str = "s";
static MELEE_UUID: &str = "a";


pub fn spell_init_system(
    mut cast_spell_init_events: EventReader<CastSpellInit>,
    mut cast_spell_fire_events: EventWriter<CastSpellFire>,
    mut player_query: Query<Entity, With<Player>>,
    mut commands: Commands
) {
    for event in &mut cast_spell_init_events {

        match event.cast_time {
            CastTime::Instant => {
                cast_spell_fire_events.send(CastSpellFire { id: event.spell_id.to_string() })
            },
            CastTime::Duration(duration) => {


                
                let player = player_query.single_mut();

                commands.entity(player).insert(Casting{
                    spell_id: event.spell_id.to_string(),
                    current_duration: Duration::ZERO,
                    total_duration: duration,
                });
            }
        }


    }
}

pub fn spell_system(
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
            "a" => {
                let mut character = character_query.single_mut();
                basic_attack(&mut character, &mut commands,  &mut other_entities)
            }
            _ => {}
        }
    }
}

fn cast_spell(
    character: &mut Transform,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    // effects: &mut ResMut<Assets<EffectAsset>>
) {
    let direction = character.forward();

    // let portal = make_particle(commands, effects);

    commands
        .spawn(

            // ParticleEffectBundle {
            //     effect: ParticleEffect::new(portal),
            //     transform: Transform::IDENTITY,
            //     ..Default::default()
            // },
            PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.2,
                stacks: 18,
                sectors: 36,
            })),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_translation(character.translation),
            ..default()
        }
    
    )
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
    player: &mut Transform,
    commands: &mut Commands,
    other_entities: &mut Query<
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

            spawn_damage_text_on_entity(commands, entity, amount);

            health.current = health.current - min(amount, health.current);

            apply_overtime(
                entity,
                commands,
                Overtime::damage_per_second(3, 5),
                &mut overtime_comp,
            );
        }
    }

    // calculate front and sides
}
