
use std::slice::Iter;

use bevy::{prelude::*, asset::LoadState};
use bevy_rapier3d::prelude::*;

use crate::Floor;

#[derive(Resource)]
struct AssetsLoading{
    assets: Vec<UntypedHandle>,
    loaded: bool
}

impl Default for AssetsLoading {
    fn default() -> Self {
        AssetsLoading{
            assets: Vec::with_capacity(2),
            loaded: false
        }
    }
}

fn setup(asset_server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    let map_glb: Handle<Scene> = asset_server.load("map.glb#Scene0");
    let map_mesh: Handle<Mesh> = asset_server.load("map.glb#Mesh0/Primitive0");

    loading.assets.push(map_glb.clone().untyped());
    loading.assets.push(map_mesh.clone().untyped());
}

fn check_assets_ready(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    assets: Res<Assets<Mesh>>,
) {

    if !loading.loaded{

        match get_load_state(&&asset_server, loading.assets.iter()) {
            LoadState::Failed => {
                // one of our assets had an error
                error!("Failed to load assets!");
            }
            LoadState::Loaded => {
                let actual_mesh = assets
                    .get(&loading.assets.get(1).unwrap().clone().typed())
                    .unwrap();
    
                let collider =
                    Collider::from_bevy_mesh(actual_mesh, &ComputedColliderShape::default()).unwrap();
                commands.spawn((
                    SceneBundle {
                        scene: loading.assets.get(0).unwrap().clone().typed(),
                        transform: Transform::from_xyz(2.0, 0.0, -5.0),
                        ..Default::default()
                    },
                    RigidBody::Fixed,
                    Floor {},
                    collider,
                ));
                
                loading.loaded = true;
                // commands.remove_resource::<AssetsLoading>();
            }
            _=> {
                info!("Not loaded yet!");
            },
    
    
            // _ => {
            //     // NotLoaded/Loading: not fully ready yet
            // }
        }
    }


}


fn get_load_state(asset_server: &Res<AssetServer>, iter: Iter<UntypedHandle>) -> LoadState {

    for handle in iter {
        match asset_server.get_load_state(handle.id()).unwrap(){
            LoadState::Loaded => {
                continue;
            }, 
            _ => {
                return asset_server.get_load_state(handle.id()).unwrap();
            }
        }
    }

    LoadState::Loaded

}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetsLoading::default())
            .add_systems(Startup, setup)
            .add_systems(Update, check_assets_ready);
    }
}
