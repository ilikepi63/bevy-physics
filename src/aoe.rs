/// Simple AOE PoC
use bevy::{core::Zeroable, input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};
use bevy_mod_raycast::{immediate::Raycast, CursorRay};

use crate::{character_controller::Player, health_bars::PrimaryCamera, Floor, lifetime::Lifetime, damage::Damage, projectile::Projectile};

pub trait GroundCastSpell {
    fn add_ground_target(&self, target: Vec3);
}

#[derive(Event)]
pub struct RayCastEvent {
    pub point: Vec3,
}

// Component that gets applied to the player when targeting the ground
#[derive(Component)]
pub struct Targeting {
    position: Vec3,
}

/// Initiate Ground Targeting
#[derive(Event)]
pub struct GroundTargetInitEvent {
    spell: Box<dyn GroundCastSpell + Send + Sync>,
} // This struct should either have intent details or spell casting

#[derive(Component)]
pub struct Pointer;

pub fn ground_targeting_system(
    mut events: EventReader<GroundTargetInitEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut player: Query<Entity, With<Player>>,
) {
    for event in events.read() {
        commands.entity(player.single()).insert(Targeting {
            position: Vec3::zeroed(),
        });
    }
}

pub fn targeting_system(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    // mut pointers: Query<&mut Transform, With<Pointer>>,
    // mut motion_evr: EventReader<MouseMotion>,
    mut gizmos: Gizmos,
    mut targeting: Query<&mut Targeting>,
    mut player: Query<(&Transform), With<Player>>,
) {
    // If we are currently targeting
    if let Ok(mut targeting) = targeting.get_single_mut() {
        // then we set the raycast
        if let Some(cursor_ray) = **cursor_ray {
            let hits = raycast.cast_ray(cursor_ray, &Default::default());

            if let Some((i, intersection)) = hits
                .iter()
                .map(|i| i.1.clone())
                .enumerate()
                .find(|(i, hit)| *i == 0 as usize)
            {
                (*targeting).position = intersection.position();

                let player = player.single().translation;

                gizmos.circle(
                    intersection.position(),
                    -intersection.normal(),
                    1.,
                    Color::GREEN,
                );

                gizmos.line(player, intersection.position(), Color::GREEN)
            }
        }
    }
}

// Used to determine if someone has fired during targeting.
pub fn targeting_click_system(
    targeting: Query<&Targeting>,
    buttons: Res<Input<MouseButton>>,
    mut player: Query<(Entity, &Transform), With<Player>>,
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(targeting) = targeting.get_single() {
        if let Ok((player, character_transform)) = player.get_single_mut() {
            if buttons.just_pressed(MouseButton::Left) {
                commands
                    .spawn(
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::UVSphere {
                                radius: 0.2,
                                stacks: 18,
                                sectors: 36,
                            })),
                            material: materials.add(Color::WHITE.into()),
                            transform: Transform::from_translation(character_transform.translation),
                            ..default()
                        },
                    )
                    .insert(Lifetime {
                        timer: Timer::from_seconds(30.0, TimerMode::Once),
                    })
                    .insert(Name::new("Bullet"))
                    .insert(Damage { amount: 10 })
                    .insert(Projectile {
                        despawn_after_hit: true,
                        speed: 0.5,
                        direction: (targeting.position - character_transform.translation).normalize() * 3.0,
                    });

                commands.entity(player).remove::<Targeting>();
            }
        }
    }
}

/// Remove this
struct GroundTargetSpell;

impl GroundCastSpell for GroundTargetSpell {
    fn add_ground_target(&self, target: Vec3) {}
}

pub fn target_init_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut writer: EventWriter<GroundTargetInitEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::F) {
        writer.send(GroundTargetInitEvent {
            spell: Box::new(GroundTargetSpell {}),
        })
    }
}

pub struct AoeTargetingPlugin;

impl Plugin for AoeTargetingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GroundTargetInitEvent>().add_systems(
            Update,
            (
                ground_targeting_system,
                target_init_system,
                targeting_system,
                targeting_click_system,
            ),
        );
    }
}
