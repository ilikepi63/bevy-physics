use bevy::{prelude::*};

use crate::{
    health_bars::{
        convert_ndc_to_percentage_values, get_sceen_transform_and_visibility, PrimaryCamera,
    },
    orbit_camera::OrbitCamera,
};

pub fn spawn_damage_text_on_entity(commands: &mut Commands, entity: Entity, value: u32) {
    commands.entity(entity).insert(AppliedDamage { value });
}

#[derive(Component)]
pub struct AppliedDamage {
    pub value: u32,
}

#[derive(Component)]
pub struct DamageTextAttach {
    pub(crate) attached_to: Entity,
}

// just to keep track so we dont spawn the same thing twice
#[derive(Component)]
pub struct DamageText {
    pub value: u32,
}

#[derive(Bundle)]
pub struct DamageTextBundle {
    pub amount: DamageText,
    pub(crate) damage: DamageTextAttach,
    #[bundle()]
    pub(crate) text: TextBundle,
}

pub struct DamageTextPlugin;

impl Plugin for DamageTextPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system(add_damage_text_to_entites_with_applied_damage);
        app.add_systems(
            Update,
            (
                spawn_damage_text_children,
                update_damage_text,
                lifetime_despawn,
            ),
        );
    }
}

fn update_damage_text(
    mut healthbars: Query<
        (
            Entity,
            &mut Text,
            &mut Style,
            &DamageTextAttach,
            &mut Visibility,
            &DamageTextLifetime,
            &DamageText,
        ),
        // Without<AppliedDamage>,
    >,
    asset_server: Res<AssetServer>,
    entites: Query<&Transform>,
    camera_q: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    orbit_camera: Query<&OrbitCamera>,
) {
    for (_hb_entity, mut hb_text, mut hb_style, hb_attach, mut hb_visibility, lifetime, text) in
        healthbars.iter_mut()
    {
        if let Ok(e_transform) = entites.get(hb_attach.attached_to) {
            let (x, y) = get_sceen_transform_and_visibility(&camera_q, e_transform, &orbit_camera);

            *hb_visibility = Visibility::Visible;

            let (x, y) = (
                convert_ndc_to_percentage_values(x),
                convert_ndc_to_percentage_values(y),
            );

            // TODO: lifetimes are a max on this as 2000ms
            // We need to make this safe
            let remaining_ms = lifetime.timer.elapsed().as_millis();

            if remaining_ms > 2000 {
                hb_style.top = Val::Percent(100.0 - y);
            } else {
                let remaining_ms_in_f32 = remaining_ms as f32 / 2000.0;

                hb_style.top = Val::Percent((100.0 - y) - (5.0 * remaining_ms_in_f32));
            }
            hb_style.left = Val::Percent(x);
            // hb_style.position.top = Val::Percent(100.0 - y);

            let current = text.value;
            let style = TextStyle {
                font_size: 20.0,
                color: Color::BLACK,
                font: asset_server.load("Rosela.ttf"),
            };

            *hb_text = Text {
                sections: [TextSection {
                    value: format!("{current}"),
                    style,
                    ..default()
                }]
                .to_vec(),
                ..default()
            };
        }
    }
}

fn spawn_damage_text_children(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    entities: Query<(Entity, &AppliedDamage, &Transform), Added<AppliedDamage>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    orbit_camera: Query<&OrbitCamera>,
) {
    for (entity, health, transform) in entities.iter() {
        let current = health.value;
        // let max = health.max();
        let bartrans = get_sceen_transform_and_visibility(&camera_q, transform, &orbit_camera);

        commands
            .spawn(
                // Healthbarbundle
                DamageTextBundle {
                    amount: DamageText {
                        value: health.value,
                    },
                    damage: DamageTextAttach {
                        attached_to: entity,
                    },
                    text: TextBundle {
                        // transform: Transform::from_xyz(bartrans.0, bartrans.1, 1.0),
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(convert_ndc_to_percentage_values(bartrans.0)),
                            top: Val::Percent(convert_ndc_to_percentage_values(100.0 - bartrans.1)),
                            ..Default::default()
                        },
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("{current}"),
                                style: TextStyle {
                                    font: asset_server.load("Rosela.ttf"),
                                    font_size: 100.0,
                                    color: Color::BLACK,
                                },
                            }],
                            ..Default::default()
                        },
                        // transform: bartrans,
                        // global_transform: bartrans.into(),
                        visibility: Visibility::Visible,
                        ..Default::default()
                    },
                },
            )
            .insert(DamageTextLifetime {
                timer: Timer::from_seconds(2.0, TimerMode::Once),
            });

        commands.entity(entity).remove::<AppliedDamage>();
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct DamageTextLifetime {
    pub timer: Timer,
}

fn lifetime_despawn(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut DamageTextLifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut entities {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
