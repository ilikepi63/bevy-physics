/// Simple AOE PoC
use bevy::{prelude::*, window::PrimaryWindow, input::mouse::MouseMotion};
use bevy_mod_raycast::{immediate::Raycast, CursorRay};

use crate::{health_bars::PrimaryCamera, Floor};

#[derive(Event)]
pub struct RayCastEvent {
    pub point: Vec3,
}

#[derive(Component)]
pub struct Pointer;

pub fn aoe_targeting_system(
    buttons: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if buttons.just_pressed(KeyCode::F) {
        info!("hello!");

        commands.spawn((
            // ParticleEffectBundle {
            //     effect: ParticleEffect::new(portal),
            //     transform: Transform::IDENTITY,
            //     ..Default::default()
            // },
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 10.0,
                    stacks: 18,
                    sectors: 36,
                })),

                material: materials.add(Color::WHITE.into()),
                transform: Transform::from_xyz(0., 0., 0.),
                ..default()
            },
            Pointer,
        ));
    }
}

pub fn targeting_system(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    mut pointers: Query<&mut Transform, With<Pointer>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut gizmos: Gizmos
) {

    // for _ in motion_evr.iter() {
        if let Some(cursor_ray) = **cursor_ray {

            let hits =  raycast.cast_ray(cursor_ray, &Default::default());

         if let Some((i , intersection)) =  hits
            .iter()
            .map(|i| i.1.clone())
            .enumerate()
            .find(|(i, hit)| *i == 0 as usize)
            // .map(|(i, hit)| (i == 0, hit)).
        {
            // let color = match is_first {
            //     true => Color::GREEN,
            //     false => Color::PINK,
            // };
            // gizmos.ray(intersection.position(), intersection.normal(), Color::GREEN);
            gizmos.circle(intersection.position(), -intersection.normal(), 1., Color::GREEN);

            // if    let Ok(mut pointer) = pointers.get_single_mut() {
            //     // get the camera info and transform
                
            //     let position = intersection.position();

            //     if pointer.translation != position {
            //         pointer.translation = position;
            //     }
        
            // }
        }
            // if let Some((entity, intersection_data)) =
            //     raycast.cast_ray(cursor_ray, &Default::default()).get(index)
            // {
            //     let position = intersection_data.position();


        

        
        
            // };
            }
    // }



}

pub struct AoeTargetingPlugin;

impl Plugin for AoeTargetingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (aoe_targeting_system, targeting_system));
    }
}
