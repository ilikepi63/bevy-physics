use bevy::prelude::*;
use bevy_rapier3d::{
    na::{Point3, UnitQuaternion, Vector3},
    rapier::{
        dynamics::{RigidBodyBuilder, RigidBodySet},
        geometry::{ColliderBuilder, ColliderSet, InteractionGroups, Ray},
        pipeline::QueryPipeline,
    },
};

use crate::MovementResource;

// use crate::{interaction_flags, resource};

// use crate::resource::InputBindings;

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

pub fn create_character_controller(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let transform = Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3{x: 0.0, y: 2.0, z: 0.0}, Vec3::Y);

    let camera_transform = transform.clone().looking_at(Vec3{x: 0.0, y: 1.0, z: 0.0}, Vec3::Y);

    let result = camera_transform.translation - (camera_transform.forward().normalize() * 20.0) ;



    // let forward_direction = camera_transform.forward();

    // let camera_position = forward_direction - 20.0;

    commands
        .spawn((
            CharacterController {
                camera_distance: 20.,
                ..Default::default()
            },
            // Transform::default(),
            // GlobalTransform::default(),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform,
                ..default()
            }, // RigidBodyBuilder::dynamic()
               //     .lock_rotations()
               //     .build(),
               // ColliderBuilder::capsule_y(0.25, 0.25)
               //     .translation(Transform::from_xyz(0.0, 0.5, 0.0))
               //     .collision_groups(InteractionGroups::all().with_groups(interaction_flags::PLAYER)),
        ));

     commands.spawn(Camera3dBundle {
            transform: Transform::from_translation(result).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });

    // Append camera to player as child.
    // commands.push_children(player_entity, &[camera_entity.unwrap()]);

    // player_entity
}

pub fn character_controller_system(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<CharacterController>)>,
    mut character_query: Query<&mut Transform, (With<CharacterController>, Without<Camera3d>)>,
    time: Res<Time>,
    mut movement: ResMut<MovementResource>,
) {
    let mut camera = camera_query.single_mut();
    let mut character = character_query.single_mut();


    let mut forward = character.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut left = character.left();
    left.y = 0.0;
    left = left.normalize();

    let speed = movement.speed;
    //Leafwing
    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * time.delta_seconds() * speed;
        character.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * time.delta_seconds() * speed;
        character.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * time.delta_seconds() * speed;
        character.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * time.delta_seconds() * speed;
        character.translation -= left * time.delta_seconds() * speed;
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

// pub fn update_player(
//     time: Res<Time>,
//     input: Res<InputBindings>,
//     query_pipeline: Res<QueryPipeline>,
//     colliders: Res<ColliderSet>,
//     mut queries: QuerySet<(
//         Query<(
//             &mut CharacterController,
//             &Transform,
//             &RigidBodyHandleComponent,
//         )>,
//         Query<&mut Transform>,
//     )>,
//     mut rigidbody_set: ResMut<RigidBodySet>,
// ) {
//     let zoom_sense = 10.0;
//     let look_sense = 1.0;

//     let move_speed = 500.0;

//     let tau = 2. * std::f32::consts::PI;

//     let delta_seconds = time.delta_seconds();

//     let movement = input.movement() * delta_seconds * move_speed;
//     let look = input.look() * delta_seconds * look_sense;
//     let zoom = input.scroll() * delta_seconds * zoom_sense;

//     let mut cam_positions: Vec<(Entity, Vec3)> = Vec::new();

//     for (mut player, transform, rigid_handle) in &mut queries.q0_mut().iter_mut() {
//         player.yaw += look.x;
//         player.yaw = (player.yaw + tau) % tau; // loop yaw within 0 - 2pi.

//         player.camera_pitch += look.y;
//         player.camera_pitch = player
//             .camera_pitch
//             .max(1f32.to_radians())
//             .min(90f32.to_radians()); // clamp camera pitch so the lowest it wont go under the character.

//         player.camera_distance += zoom;
//         player.camera_distance = player.camera_distance.max(5.).min(30.);

//         let fwd = transform.forward().normalize();
//         let right = Vec3::cross(fwd, Vec3::unit_y()).normalize();

//         let fwd = fwd * movement.y;
//         let right = right * movement.x;

//         let direction = Vec3::from(fwd + right);

//         let rigid = rigidbody_set.get_mut(rigid_handle.handle()).unwrap();

//         let origin = Point3::new(
//             transform.translation.x,
//             transform.translation.y + 0.3,
//             transform.translation.z,
//         );
//         let ray = Ray::new(origin, Vector3::new(0.0, -1.0, 0.0));

//         if let Some((_, _, intersection)) = query_pipeline.cast_ray(
//             &colliders,
//             &ray,
//             0.5,
//             InteractionGroups::all().with_mask(!interaction_flags::PLAYER),
//         ) {
//             let normal = Vector3::new(
//                 intersection.normal.x,
//                 intersection.normal.y,
//                 intersection.normal.z,
//             );
//             let angle = Vector3::dot(&normal, &Vector3::new(0.0, 1.0, 0.0))
//                 .acos()
//                 .to_degrees();

//             // TODO WT: Max slope angle should be on CharacterController.
//             player.grounded = angle < 44.0;
//         } else {
//             player.grounded = false;
//         }

//         if player.grounded {
//             let mut linvel: Vector3<f32> = *rigid.linvel();
//             linvel.x = direction.x;
//             linvel.z = direction.z;

//             rigid.set_linvel(linvel, true);

//             if input.do_jump() {
//                 rigid.apply_impulse(Vector3::new(0.0, 10.0, 0.0), true);
//             }
//         }

//         let mut position = *rigid.position();
//         position.rotation = UnitQuaternion::new(Vector3::y() * -player.yaw);
//         rigid.set_position(position, false);

//         if let Some(camera_entity) = player.camera_entity {
//             let cam_pos = Vec3::new(0., player.camera_pitch.cos(), -player.camera_pitch.sin())
//                 .normalize()
//                 * player.camera_distance;
//             cam_positions.push((camera_entity, cam_pos));
//         }
//     }

//     for (camera_entity, cam_pos) in cam_positions.iter() {
//         if let Ok(mut cam_trans) = queries.q1_mut().get_mut(*camera_entity) {
//             cam_trans.translation = *cam_pos;

//             let look = Mat4::face_toward(
//                 cam_trans.translation,
//                 Vec3::zero(),
//                 Vec3::new(0.0, 1.0, 0.0),
//             );
//             cam_trans.rotation = look.to_scale_rotation_translation().1;
//         }
//     }
// }
