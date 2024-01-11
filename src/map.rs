use bevy::prelude::*;
// use bevy_rapier3d::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::Floor;

pub fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
) {
    // let map_glb: Handle<Scene> = asset_server.load("character_controller_demo.glb#Scene0");
    // let map_mesh: Handle<Mesh> = asset_server.load("map.glb#Mesh0/Primitive0");

    commands.spawn((
        SceneBundle {
            // scene: asset_server.load("character_controller_demo.glb#Scene0"),
            scene: asset_server.load("map.glb#Scene0"),
            // transform: Transform::from_rotation(Quat::from_rotation_y(-std::f32::consts::PI * 0.5)),
            ..default()
        },
        AsyncSceneCollider::new(Some(ComputedCollider::ConvexDecomposition(
            VHACDParameters::default(),
        )))
        // Make the arms heavier to make it easier to stand upright
        .with_density_for_name("armL_mesh", 5.0)
        .with_density_for_name("armR_mesh", 5.0),
        RigidBody::Static,
        Floor{}
    ));

    // loading.assets.push(map_glb.clone().untyped());
    // loading.assets.push(map_mesh.clone().untyped());
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map);
        // .add_systems(Update, check_assets_ready);
    }
}
