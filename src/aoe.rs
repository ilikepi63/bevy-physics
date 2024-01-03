/// Simple AOE PoC
use bevy::{prelude::*, window::PrimaryWindow, ecs::world};

use crate::{health_bars::PrimaryCamera, lifetime::Lifetime, Floor};

pub fn aoe_targeting_system(
    buttons: Res<Input<KeyCode>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    plane_q: Query<&GlobalTransform, With<Floor>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

) {
    if buttons.just_pressed(KeyCode::F) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // Ditto for the ground plane's transform
    let ground_transform = plane_q.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = primary_window.single();

    // check if the cursor is inside the window and get its position
    let Some(cursor_position) = window.cursor_position() else {
        // if the cursor is not inside the window, we can't do anything
        return;
    };

    // Mathematically, we can represent the ground as an infinite flat plane.
    // To do that, we need a point (to position the plane) and a normal vector
    // (the "up" direction, perpendicular to the ground plane).

    // We can get the correct values from the ground entity's GlobalTransform
    let plane_origin = ground_transform.translation();
    let plane_normal = ground_transform.up();

    // Ask Bevy to give us a ray pointing from the viewport (screen) into the world
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        // if it was impossible to compute for whatever reason; we can't do anything
        return;
    };

    // do a ray-plane intersection test, giving us the distance to the ground
    let Some(distance) = ray.intersect_plane(plane_origin, plane_normal) else {
        // If the ray does not intersect the ground
        // (the camera is not looking towards the ground), we can't do anything
        return;
    };

    // use the distance to compute the actual point on the ground in world-space
    let global_cursor = ray.get_point(distance);


    // to compute the local coordinates, we need the inverse of the plane's transform
    let inverse_transform_matrix = ground_transform.compute_matrix().inverse();
    let local_cursor = inverse_transform_matrix.transform_point3(global_cursor);

    // we can discard the Y coordinate, because it should always be zero
    // (our point is supposed to be on the plane
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
                        transform: Transform::from_xyz(local_cursor.x, 1.0, local_cursor.z),
                        ..default()
                    },
                );
    }
}

pub struct AoeTargetingPlugin;

impl Plugin for AoeTargetingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(aoe_targeting_system);
    }
}
