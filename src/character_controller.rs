use bevy::prelude::*;
use bevy::ui::camera_config::UiCameraConfig;
// use bevy_rapier3d::prelude::*;
use bevy_xpbd_3d::{
    math::{Scalar, Vector},
    prelude::*,
};

use crate::{
    controller::CharacterControllerBundle,
    health::Health,
    health_bars::PrimaryCamera,
    orbit_camera::{self},
};

// use crate::{interaction_flags, resource};

// use crate::resource::InputBindings;

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component)]
pub struct CharacterDirection {
    pub forward: Vec3,
    pub right: Vec3,
}

#[derive(Component)]
pub struct CharacterTranslation(pub Vec3);

pub fn create_character_controller(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    let transform = Transform::from_xyz(-2.0, 1.5, 5.0).looking_at(
        Vec3 {
            x: 0.0,
            y: 3.0,
            z: 0.0,
        },
        Vec3::Y,
    );

    let camera_transform = transform.looking_at(
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Vec3::Y,
    );

    let result = camera_transform.translation - (camera_transform.forward().normalize() * 20.0);

    // commands.spawn((
    //     Collider::capsule(1.0, 0.4),

    //     // Transform::default(),
    //     // GlobalTransform::default(),
    //     PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Capsule {
    //             radius: 0.4,
    //             ..default()
    //         })),
    //         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //         transform: Transform::from_xyz(0.0, 6.0, 0.0),
    //         ..default()
    //     },
    //     RigidBody::Kinematic,
    //     ControllerGravity(Vector::NEG_Y * 9.81 * 2.0)
    //     // RigidBody::Dynamic,
    //     // GravityScale(1.0),
    //     // Velocity::zero(), // RigidBodyBuilder::dynamic().lock_rotations().build(),
    // ));

    let transform = Transform::from_xyz(0.0, 1.5, 0.0);

    let character_translation = CharacterTranslation(transform.translation);

    commands.spawn((
        // Transform::default(),
        // GlobalTransform::default(),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.4,
                ..default()
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform,
            ..default()
        },
        Health {
            current: 100,
            max: 100,
        },
        Player {},
        CharacterControllerBundle::new(Collider::capsule(1.0, 0.4), Vector::NEG_Y * 9.81 * 2.0)
            .with_movement(100.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
        CharacterDirection {
            forward: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            right: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        },
        character_translation, // Collider::ball(1.0),
                               // RigidBody::Dynamic,
                               // GravityScale(1.0),
                               // Velocity::zero(), // RigidBodyBuilder::dynamic().lock_rotations().build(),
    ));

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(result),
            ..default()
        })
        .insert(orbit_camera::OrbitCamera {
            center: transform.translation,
            ..Default::default()
        })
        .insert(UiCameraConfig { show_ui: true })
        .insert(PrimaryCamera);
}

// pub fn character_controller_system(
//     keyboard: Res<Input<KeyCode>>,
//     mouse_button_input: Res<Input<MouseButton>>,
//     mut query: Query<&mut OrbitCamera>,
//     mut character_query: Query<(&mut Transform), With<CharacterController>>,

//     time: Res<Time>,
//     mut movement: ResMut<MovementResource>,
//     // camera_query: Query<(&Transform), (With<Camera>, Changed<Camera>)>,
// ) {
//     let mut camera = query.single_mut();
//     let (mut character) = character_query.single_mut();

//     let mut forward = character.forward();
//     forward.y = 0.0;
//     forward = forward.normalize();

//     let mut left = character.left();
//     left.y = 0.0;
//     left = left.normalize();

//     let speed = movement.speed;

//     //Leafwing
//     if keyboard.pressed(KeyCode::W) {
//         camera.center += forward * time.delta_seconds() * speed;
//         character.translation += forward * time.delta_seconds() * speed;
//         // velocity.linvel = forward * speed;
//     }
//     if keyboard.pressed(KeyCode::S) {
//         camera.center -= forward * time.delta_seconds() * speed;
//         character.translation -= forward * time.delta_seconds() * speed;
//         // vector_direction -= forward;
//     }
//     if keyboard.pressed(KeyCode::A) {
//         camera.center += left * time.delta_seconds() * speed;
//         character.translation += left * time.delta_seconds() * speed;
//         // vector_direction += left;
//     }
//     if keyboard.pressed(KeyCode::D) {
//         camera.center -= left * time.delta_seconds() * speed;
//         character.translation -= left * time.delta_seconds() * speed;
//         // vector_direction -= left;
//     }

//     // Stop moving

//     // if keyboard.just_released(KeyCode::W) {
//     //     // camera.center += forward * time.delta_seconds() * speed;
//     //     // character.translation += forward * time.delta_seconds() * speed;
//     //     velocity.linvel = velocity.linvel - forward * 1000.0;
//     // }
//     // if keyboard.just_released(KeyCode::S) {
//     //     // camera.center -= forward * time.delta_seconds() * speed;
//     //     velocity.linvel = velocity.linvel + forward * 1000.0;
//     // }
//     // if keyboard.just_released(KeyCode::A) {
//     //     // camera.center += left * time.delta_seconds() * speed;
//     //     velocity.linvel = velocity.linvel - left * 1000.0;
//     // }
//     // if keyboard.just_released(KeyCode::D) {
//     //     // camera.center -= left * time.delta_seconds() * speed;
//     //     velocity.linvel = velocity.linvel + left * 1000.0;
//     // }

//     // Something else

//     if keyboard.pressed(KeyCode::ShiftLeft) {
//         movement.speed = 20.0
//     }

//     if keyboard.just_released(KeyCode::ShiftLeft) {
//         movement.speed = 7.0;
//     }

//     if mouse_button_input.pressed(camera.rotate_button) {
//         let mut camera_direction = camera.direction;

//         camera_direction.y = forward.y;
//         // camera_direction.x = forward.x;

//         character.look_to(camera_direction, Vec3::Y)
//     }
// }

pub fn update_character_transform(
    mut character_direction: Query<(&mut Transform, &CharacterDirection)>,
) {
    for (mut transform, direction) in character_direction.iter_mut() {
        transform.look_to(direction.forward, Vec3::Y);
    }
}
