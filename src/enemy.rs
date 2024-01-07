use bevy::prelude::*;

use crate::{health_bars::{
    get_sceen_transform_and_visibility, HealthBarAttach, HealthBarBundle, PrimaryCamera,
}, hit_box::HitBox, health::Health};

#[derive(Debug, Component)]
pub struct Enemy;

// pub fn setup_enemy(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let transform = Transform::from_xyz(-20.0, 1.5, 15.0).looking_at(
//         Vec3 {
//             x: 0.0,
//             y: 1.0,
//             z: 0.0,
//         },
//         Vec3::Y,
//     );

//     let default_capsule =Mesh::from(shape::Capsule::default()); 

//     // let capsule = Collider::capsule(Vect::, b, 1.0);

//     commands
//         .spawn(PbrBundle {
//             mesh: meshes.add(default_capsule),
//             material: materials.add(Color::RED.into()),
//             transform,
//             ..Default::default()
//         })
//         .insert(Health {
//             current: 150,
//             max: 150,
//         })
//         .insert(Enemy).insert(HitBox{
//             radius: 1,
//             height: 1
//         });
// }


pub fn spawner(
    mut commands: Commands,
    enemies: Query<&Enemy>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if enemies.is_empty() {

        for i in 1..=3 {
            let transform = Transform::from_xyz(-20.0 * i as f32, 1.5, 15.0).looking_at(
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


    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawner);
        // app.add_system(enemy_health_system);
    }
}
