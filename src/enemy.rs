use bevy::prelude::*;
use bevy_rapier3d::{rapier::geometry::ColliderBuilder, dynamics::RigidBody, geometry::Collider};

use crate::{health_bars::{
    get_sceen_transform_and_visibility, HealthBarAttach, HealthBarBundle, PrimaryCamera,
}, hit_box::HitBox, health::Health};

#[derive(Debug, Component)]
pub struct Enemy;

pub fn setup_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let transform = Transform::from_xyz(-20.0, 1.5, 15.0).looking_at(
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Vec3::Y,
    );

    let default_capsule =Mesh::from(shape::Capsule::default()); 

    // let capsule = Collider::capsule(Vect::, b, 1.0);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(default_capsule),
            material: materials.add(Color::RED.into()),
            transform,
            ..Default::default()
        })
        .insert(Health {
            current: 150,
            max: 150,
        })
        .insert(Enemy).insert(HitBox{
            radius: 1,
            height: 1
        });
}

// pub fn enemy_health_system(
//     entities: Query<(Entity, &Transform), With<Enemy>>,
//     camera_q: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let (camera, camera_global_transform) = camera_q.single();

//     for (entity, transform) in entities.iter() {
//         // spawn the healthbar as a child
//         let style = TextStyle {
//             // font: hres.font.clone(),
//             font_size: 12.0,
//             color: Color::RED,
//             ..Default::default()
//         };
//         let current = 100;
//         let max = 100;
//         let text = Text::from_section(format!("{current}/{max}"), style);
//         let health_bar_transform = camera
//             .world_to_viewport(camera_global_transform, transform.translation)
//             .map(|pos| {
//                 // let y = pos.y + 3;

//                 Transform::from_xyz(pos.x + 6.0, pos.y + 6.0, 1.)
//             });

//         if let Some(health_bar_transform) = health_bar_transform {
//             info!("Spawning Health Bars!!");

//             let text = "Hello world!";

//             commands.spawn(
//                 TextBundle::from_section(
//                     text,
//                     TextStyle {
//                         font_size: 100.0,
//                         color: Color::WHITE,
//                         ..default()
//                     },
//                 ) // Set the alignment of the Text
//                 .with_text_alignment(TextAlignment::Center)
//                 // Set the style of the TextBundle itself.
//                 .with_style(Style {
//                     position_type: PositionType::Absolute,
//                     position: UiRect {
//                         left: Val::Px(5.0),
//                         top: Val::Px(5.0),
//                         ..Default::default()
//                     },
//                     ..default()
//                 }),
//             );
//         } else {
//             error!("Could not get coordinates of stuff");
//         }
//     }
// }

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_enemy);
        // app.add_system(enemy_health_system);
    }
}
