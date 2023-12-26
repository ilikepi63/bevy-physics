use std::cmp::min;
use std::f32::consts::PI;
use std::process::Command;

use bevy::ecs::event::ManualEventReader;
use bevy::ecs::query::Without;
use bevy::input::mouse::MouseMotion;
use bevy::log::info;
use bevy::pbr::{AmbientLight, CascadeShadowConfigBuilder};
use bevy::prelude::{
    default, shape, App, Assets, Color, Commands, Component, DirectionalLightBundle, Entity,
    EventReader, Input, KeyCode, Mesh, MouseButton, Name, PbrBundle, Quat, Query, Res, ResMut,
    Resource, StandardMaterial, Transform, Vec3, With,
};
use bevy::render::camera::Camera;
use bevy::text::{TextAlignment, TextStyle};
use bevy::time::{Timer, TimerMode};
use bevy::transform::components::GlobalTransform;
use bevy::ui::node_bundles::TextBundle;
use bevy::ui::{PositionType, Style, UiRect, Val};
use bevy::window::{CursorGrabMode, PrimaryWindow, Window};
use bevy::DefaultPlugins;
// use bevy_inspector_egui::egui::Key;
use bevy_rapier3d::prelude::{
    ActiveEvents, Collider, CollisionEvent, ContactForceEvent, ContactForceEventThreshold,
    NoUserData, RapierPhysicsPlugin, RigidBody, Velocity,
};
use character_controller::{
    character_controller_system, create_character_controller, CharacterController, Player,
};
use damage::Damage;
use damage_text::{spawn_damage_text_on_entity, AppliedDamage, DamageTextPlugin};
use enemy::EnemyPlugin;
use health::{health_system, Health};
use health_bars::{HealthBar, HealthBarPlugin};
use lifetime::{Lifetime, LifetimePlugin};
use projectile::{Projectile, ProjectilePlugin};

pub mod character_controller;
// pub mod health;
mod damage;
mod damage_text;
pub mod enemy;
mod health;
pub mod health_bars;
pub mod hit_box;
mod lifetime;
pub mod orbit_camera;
pub mod projectile;
pub mod utils;
mod auras;

pub const PLAYER: u16 = 0b1;
pub const STATIC_GEOMETRY: u16 = 0b10;

#[derive(Resource)]
pub struct MovementResource {
    pub speed: f32,
}

impl Default for MovementResource {
    fn default() -> Self {
        Self { speed: 7.0 }
    }
}

// Runs on startup, adds a ground plane
fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(5000.0).into()),
            material: materials.add(Color::DARK_GREEN.into()),
            ..default()
        })
        .insert(Collider::cuboid(5000.0, 0.1, 5000.0))
        .insert(Floor {});
}

#[derive(Component)]
struct Despawnable {}

#[derive(Component)]
struct Floor {}

// HEALTH BAR

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .init_resource::<InputState>()
        .init_resource::<MovementResource>()
        .add_startup_system(create_character_controller)
        .add_plugin(orbit_camera::OrbitCameraPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(LifetimePlugin)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_world)
        .add_system(character_controller_system)
        .add_system(cursor_grab)
        .add_system(on_mouse_shoot)
        // .add_system(display_events)
        .add_plugin(HealthBarPlugin)
        .add_system(health_system)
        .add_plugin(ProjectilePlugin)
        .add_plugin(DamageTextPlugin)
        .add_system(basic_attack)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // light

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 10.0,
    });
}

#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

fn cursor_grab(
    buttons: Res<Input<MouseButton>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if buttons.just_pressed(MouseButton::Right) {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }

        if buttons.just_released(MouseButton::Right) {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    } else {
        // warn!("Primary window not found for `cursor_grab`!");
    }
}

fn on_mouse_shoot(
    buttons: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<CharacterController>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if buttons.just_pressed(KeyCode::Q) {
        let camera = camera_query.single_mut();

        let direction = camera.forward();

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.2,
                    stacks: 18,
                    sectors: 36,
                })),
                material: materials.add(Color::BLACK.into()),
                transform: Transform::from_translation(camera.translation),
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
}

fn basic_attack(
    buttons: Res<Input<KeyCode>>,
    player: Query<&Transform, With<Player>>,
    mut commands: Commands,

    // This is a big query right now
    mut other_entities: Query<(Entity, &Transform, &mut Health), (With<Health>, Without<Player>)>,
) {
    if buttons.just_pressed(KeyCode::R) {
        let player = player.single().translation;

        for (entity, transform, mut health) in other_entities.iter_mut() {
            // check if the entity is within radius
            let distance = ((utils::safe_minus(transform.translation.z, player.z)).powi(2)
                + (utils::safe_minus(transform.translation.x, player.x)).powi(2))
            .sqrt();

            // this just takes into account distance along the xz plane
            if distance < 2.0 {
                let amount = 10;

                spawn_damage_text_on_entity(&mut commands, entity, amount);

                health.current = health.current - min(amount, health.current);
            }
        }

        // calculate front and sides
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    // _contact_force_events: EventReader<ContactForceEvent>,
    mut commands: Commands,
    mut health_entities: Query<(Entity, &mut Health)>,
    damage_dealer_entities: Query<(Entity, &Damage)>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(first_entity, second_entity, _) => {
                let mut health_ent = health_entities.get_mut(*first_entity).ok();

                if let None = health_ent {
                    health_ent = health_entities.get_mut(*second_entity).ok();
                }

                let (hit_ent, mut health) = if let Some(ent) = health_ent {
                    ent
                } else {
                    // debug!("No entity with health!");
                    return;
                };

                let damage_ent = damage_dealer_entities
                    .get(*first_entity)
                    .map_err(|_| damage_dealer_entities.get(*second_entity))
                    .ok();

                if let Some((_, Damage { amount })) = damage_ent {
                    info!("Applying damage!");

                    commands
                        .entity(hit_ent)
                        .insert(AppliedDamage { value: *amount });

                    // health.current = health.current - amount;
                };
            }
            _ => {}
        }
    }

    // for contact_force_event in contact_force_events.iter() {
    //     println!("Received contact force event: {contact_force_event:?}");
    // }
}
