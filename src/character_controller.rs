use bevy::prelude::*;
use bevy::ui::camera_config::UiCameraConfig;
use bevy_rapier3d::{rapier::{geometry::{ColliderBuilder, InteractionGroups}, dynamics::RigidBodyBuilder}, control::KinematicCharacterController};

use crate::{
    health_bars::{PrimaryCamera},
    orbit_camera::{self, OrbitCamera},
    MovementResource, health::Health,
};

// use crate::{interaction_flags, resource};

// use crate::resource::InputBindings;

#[derive(Debug, Component)]
pub struct Player;


#[derive(Debug, Component)]
pub struct CharacterController {
    pub yaw: f32,

    pub camera_distance: f32,
    pub camera_pitch: f32,

    pub grounded: bool,
}

impl Default for CharacterController {
    fn default() -> Self {
        CharacterController {
            yaw: 0.0,
            camera_distance: 20.,
            camera_pitch: 30.0f32.to_radians(),
            grounded: true,
        }
    }
}

#[derive(Debug, Component)]
pub struct CharacterDirection(pub Vec3);

pub fn create_character_controller(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let transform = Transform::from_xyz(-2.0, 1.5, 5.0).looking_at(
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Vec3::Y,
    );

    let camera_transform = transform.clone().looking_at(
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Vec3::Y,
    );

    let result = camera_transform.translation - (camera_transform.forward().normalize() * 20.0);

    commands.spawn((
        KinematicCharacterController::default(),
        CharacterController {
            camera_distance: 20.,
            ..Default::default()
        },
        CharacterDirection(Vec3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        }),
        // Transform::default(),
        // GlobalTransform::default(),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform,
            ..default()
        },
        Health {
            current: 100,
            max: 100,
        },
        Player{},
        // RigidBodyBuilder::dynamic().lock_rotations().build(),

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

pub fn character_direction_system(
    camera_query: Query<(&Transform, &Camera), (With<Camera3d>, Changed<OrbitCamera>)>,
    mut character: Query<&mut CharacterDirection, With<CharacterDirection>>,
) {
    let (transform, _) = camera_query.single();

    // let camera_direction = transform.forward();

    character.single_mut().0 = transform.forward();
}

pub fn character_controller_system(
    keyboard: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut query: Query<&mut OrbitCamera>,
    // mut query: Query<(&OrbitCamera, &mut Transform), (Changed<OrbitCamera>, With<Camera>)>,
    mut character_query: Query<(&mut Transform, &CharacterDirection), With<CharacterController>>,

    time: Res<Time>,
    mut movement: ResMut<MovementResource>,
    // camera_query: Query<(&Transform), (With<Camera>, Changed<Camera>)>,
) {
    let mut camera = query.single_mut();
    let (mut character, _character_direction) = character_query.single_mut();

    let mut forward = character.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = character.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = movement.speed;
    //Leafwing
    if keyboard.pressed(KeyCode::W) {
        camera.center += forward * time.delta_seconds() * speed;
        character.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.center -= forward * time.delta_seconds() * speed;
        character.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.center += left * time.delta_seconds() * speed;
        character.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.center -= left * time.delta_seconds() * speed;
        character.translation -= left * time.delta_seconds() * speed;
    }

    if keyboard.pressed(KeyCode::ShiftLeft) {
        movement.speed = 20.0
    }

    if keyboard.just_released(KeyCode::ShiftLeft) {
        movement.speed = 7.0;
    }

    if mouse_button_input.pressed(camera.rotate_button) {
        let mut camera_direction = camera.direction;

        camera_direction.y = forward.y;
        // camera_direction.x = forward.x;

        character.look_to(camera_direction, Vec3::Y)
    }
}
