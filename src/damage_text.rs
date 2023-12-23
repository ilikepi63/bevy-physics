use bevy::{prelude::*, transform};

use crate::{health::Health, orbit_camera::OrbitCamera, health_bars::{PrimaryCamera, get_sceen_transform_and_visibility, convert_ndc_to_percentage_values}};

#[derive(Component)]
pub struct AppliedDamage{
    pub value: u32
}

#[derive(Component)]
pub struct DamageTextAttach {
    pub(crate) attached_to: Entity,
}

// just to keep track so we dont spawn the same thing twice
#[derive(Component)]
pub struct DamageText {
    pub value: u32
}

#[derive(Bundle)]
pub struct DamageTextBundle {
    pub(crate) damage: DamageTextAttach,
    #[bundle]
    pub(crate) text: TextBundle,
}

pub struct DamageTextPlugin;

impl Plugin for DamageTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_damage_text_to_entites_with_applied_damage);
        app.add_system(spawn_damage_text_children);
        app.add_system(update_damage_text);
        // app.add_system(despawn_unattached_healthbars);
    }
}

fn add_damage_text_to_entites_with_applied_damage(
    mut commands: Commands,
    entities: Query<(Entity, &AppliedDamage),(Without<DamageText>)>,
) {
    for (entity, ap) in entities.iter() {
        if let Some(mut ec) = commands.get_entity(entity) {
            ec.insert(DamageText{
                value: ap.value
            });
        }
    }
}

// we despawn after a timer
// fn despawn_unattached_healthbars(
//     mut commands: Commands,
//     healthbars: Query<(Entity, &HealthBarAttach), Without<HealthBar>>,
//     entites: Query<(&Health, &Transform), With<HealthBar>>,
// ) {
//     for (hb_entity, attach) in healthbars.iter() {
//         // despawn the healthbar
//         if let Err(_) = entites.get(attach.attached_to) {
//             if let Some(ec) = commands.get_entity(hb_entity) {
//                 ec.despawn_recursive()
//             }
//         }
//     }
// }

fn update_damage_text(
    mut healthbars: Query<
        (
            Entity,
            &mut Text,
            &mut Style,
            &mut Transform,
            &DamageTextAttach,
            &mut Visibility,
        ),
        Without<DamageText>,
    >,
    asset_server: Res<AssetServer>,
    entites: Query<(&AppliedDamage, &Transform, &DamageText)>,
    camera_q: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    orbit_camera: Query<(&OrbitCamera)>
) {
    for (_hb_entity, mut hb_text, mut hb_style, mut hb_transform, hb_attach, mut hb_visibility) in
        healthbars.iter_mut()
    {
        if let Ok((e_health, e_transform, e_bar)) = entites.get(hb_attach.attached_to) {
            let (x,y) = get_sceen_transform_and_visibility(&camera_q, e_transform, &orbit_camera);
            // *hb_transform = bartrans;
            *hb_visibility = Visibility::Visible;


            let (x, y) = (convert_ndc_to_percentage_values(x), convert_ndc_to_percentage_values(y));

            hb_style.position.left = Val::Percent(x);
            hb_style.position.top = Val::Percent(100.0 - y);


            let current = e_health.value;
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
    orbit_camera: Query<(&OrbitCamera)>
) {
    for (entity, health, transform) in entities.iter() {
        info!("Yes");
        let current = health.value;
        // let max = health.max();
        let bartrans = get_sceen_transform_and_visibility(&camera_q, transform, &orbit_camera);

        commands.spawn(
            // Healthbarbundle
            DamageTextBundle {
                damage: DamageTextAttach {
                    attached_to: entity,
                },
                text: TextBundle {
                    // transform: Transform::from_xyz(bartrans.0, bartrans.1, 1.0),
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Percent(convert_ndc_to_percentage_values(bartrans.0)),
                            top: Val::Percent(convert_ndc_to_percentage_values(100.0 - bartrans.1)),
                            ..Default::default()
                        },
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
        );

        // commands.entity(entity).insert(HealthBar::default());
    }
}
