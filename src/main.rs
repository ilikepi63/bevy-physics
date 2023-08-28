use std::f32::consts::PI;

use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::{
    default, shape, App, Assets, Camera3d, Camera3dBundle, Color, Commands, Component,
    DirectionalLight, DirectionalLightBundle, Entity, EulerRot, EventReader, Input, KeyCode, Mesh,
    MouseButton, Name, PbrBundle, Quat, Query, Res, ResMut, Resource, StandardMaterial, Time,
    Transform, Vec3, With, Without,
};
use bevy::reflect::Reflect;
use bevy::time::{Timer, TimerMode};
use bevy::window::{CursorGrabMode, PrimaryWindow, Window};
use bevy::DefaultPlugins;
use bevy_inspector_egui::egui::Key;
use bevy_rapier3d::prelude::{
    ActiveEvents, Collider, CollisionEvent, ContactForceEvent, ContactForceEventThreshold,
    NoUserData, RapierPhysicsPlugin, RigidBody, Velocity,
};
use bullet::{Bullet, BulletPlugin, Lifetime};
use character_controller::{character_controller_system, create_character_controller, CharacterController};

pub mod character_controller;
// pub mod resource;
// pub mod interaction_flags;
pub mod bullet;
// pub mod health;

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
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..default()
        })
        .insert(Collider::cuboid(5000.0, 0.1, 5000.0))
        .insert(Floor {});

    for i in 0..100 {
        spawn_cube(
            1.0,
            (i as f32) + 0.1,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
        // if i % 2 == 0 {
        //     spawn_ball(1.0, i as f32 * 10.0, &mut commands, &mut meshes, &mut materials)
        // }else{
        //     spawn_cube(1.0, i as f32 * 10.0, &mut commands, &mut meshes, &mut materials)
        // };
    }
}

#[derive(Component)]
struct Despawnable {}

#[derive(Component)]
struct Floor {}

fn spawn_cube(
    size: f32,
    height: f32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, height, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(size / 2.0, size / 2.0, size / 2.0))
        .insert(Despawnable {});
}

fn spawn_ball(
    size: f32,
    height: f32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: size,
                stacks: 18,
                sectors: 36,
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, height, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(size));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(BulletPlugin)
        .init_resource::<InputState>()
        .init_resource::<MovementResource>()
        .add_startup_system(create_character_controller)
        // .add_plugin(PlayerPlugin)
        // .add_startup_system(setup_camera)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_world)
        .add_system(character_controller_system)
        // .add_system(camera_controls)
        .add_system(mouse_camera_controls)
        .add_system(cursor_grab)
        .add_system(on_mouse_shoot)
        .add_system(display_events)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });
}

#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

fn mouse_camera_controls(
    mut character_query: Query<&mut Transform, (With<CharacterController>, Without<Camera3d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<CharacterController>)>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {

    let window = primary_window.get_single();

    let sensitivity = 0.00012;

    if let Ok(window) = window {
        if window.cursor.grab_mode == CursorGrabMode::None {
            return; // don't do anything if the cursor is not locked
        }

        if let (Ok(mut character_transform ),Ok(mut camera_transform)) = (character_query.get_single_mut(), camera_query.get_single_mut()) {
            for ev in state.reader_motion.iter(&motion) {

                let (mut yaw, mut pitch, _) = character_transform.rotation.to_euler(EulerRot::YXZ);

                let window_scale = window.height().min(window.width());

                yaw -= (sensitivity * ev.delta.x * window_scale).to_radians();

                pitch = pitch.clamp(-1.54, 1.54);

                let rotation =  Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);

                // Order is important to prevent unintended roll
                character_transform.rotation = rotation;    

                // yaw -
                // pitch -= ev.delta.y.to_radians();
                // yaw -= (0.0001 * sensitivity * ev.delta.x * window_scale).to_radians();
                // pitch = pitch.clamp(-1.54, 1.54);

                let yaw = ev.delta.x.to_radians();
                let pitch = ev.delta.y.to_radians();

                camera_transform.rotate_around(character_transform.translation, Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch));

                // camera_tranform.rotation = 
            }
        }

        // let character_transform = character_query.get_single();

        // for mut transform in character_query.iter_mut() {
        //     for ev in state.reader_motion.iter(&motion) {

        //         let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

        //         let window_scale = window.height().min(window.width());
        //         // pitch -= (sensitivity * ev.delta.y * window_scale).to_radians();
        //         yaw -= (sensitivity * ev.delta.x * window_scale).to_radians();

        //         pitch = pitch.clamp(-1.54, 1.54);

        //         // Order is important to prevent unintended roll
        //         transform.rotation =
        //             Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        //     }
        // }

        // for mut transform in camera_query.iter_mut() {
        //     for ev in state.reader_motion.iter(&motion) {

        //         let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

        //         let window_scale = window.height().min(window.width());
        //         pitch -= (sensitivity * ev.delta.y * window_scale).to_radians();
        //         yaw -= (sensitivity * ev.delta.x * window_scale).to_radians();

        //         pitch = pitch.clamp(-1.54, 1.54);

        //         // Order is important to prevent unintended roll
        //         transform.rotation =
        //             Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        //     }
        // }
    } else {
        println!("No primary window..");
    }
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
    mut movement: ResMut<MovementResource>,
) {
    let mut camera = camera_query.single_mut();

    let mut forward = camera.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = camera.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = movement.speed;
    //Leafwing
    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    // if keyboard.pressed(KeyCode::Q) {
    //     camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
    // }
    // if keyboard.pressed(KeyCode::E) {
    //     camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
    // }
    if keyboard.pressed(KeyCode::LShift) {
        movement.speed = 20.0
    }

    if keyboard.just_released(KeyCode::LShift) {
        movement.speed = 7.0;
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
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

        println!("Ball going woosh: {:?}", direction);

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
                timer: Timer::from_seconds(20.0, TimerMode::Once),
            })
            .insert(Velocity::linear(direction * 100.0))
            .insert(Bullet {
                direction,
                speed: 200.0,
            })
            .insert(Name::new("Bullet"))
            .insert(RigidBody::Dynamic)
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(ContactForceEventThreshold(30.0))
            .insert(Collider::ball(1.0));
    }
}
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    mut commands: Commands,
    entities: Query<Entity, With<Despawnable>>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(first_entity, second_entity, _) => {
                match entities.get(*first_entity) {
                    Ok(e) => {
                        commands.entity(e).despawn();
                    }
                    Err(_e) => {}
                }

                match entities.get(*second_entity) {
                    Ok(e) => {
                        commands.entity(e).despawn();
                    }
                    Err(_e) => {}
                }
            }
            _ => {}
        }
    }

    // for contact_force_event in contact_force_events.iter() {
    //     println!("Received contact force event: {contact_force_event:?}");
    // }
}
