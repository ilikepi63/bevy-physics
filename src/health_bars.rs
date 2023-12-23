use bevy::prelude::*;

use crate::health::Health;


impl HealthTrait for Health {
    fn current(&self) -> u32 {
        self.current
    }

    fn max(&self) -> u32 {
        self.max
    }
}

#[derive(Component)]
pub struct PrimaryCamera;

pub trait HealthTrait {
    fn current(&self) -> u32;
    fn max(&self) -> u32;
}

#[derive(Component)]
pub struct HealthBarAttach {
    pub(crate) attached_to: Entity,
}

// just to keep track so we dont spawn the same thing twice
#[derive(Component)]
pub struct HealthBar {
    pub offset: Vec2,
    pub size: f32,
    pub color: Color,
}
impl Default for HealthBar {
    fn default() -> Self {
        Self {
            offset: Vec2::new(0.0, 0.0),
            size: 10.,
            color: Color::BLACK,
        }
    }
}

#[derive(Bundle)]
pub struct HealthBarBundle {
    pub(crate) healthbar: HealthBarAttach,
    #[bundle]
    pub(crate) text: TextBundle,
}

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_healthbars_to_entites_with_health);
        app.add_system(spawn_health_bar_children);
        app.add_system(update_healthbars);
        app.add_system(despawn_unattached_healthbars);
    }
}

fn add_healthbars_to_entites_with_health(
    mut commands: Commands,
    entities: Query<Entity, (With<Health>, Without<HealthBar>)>,
) {
    for entity in entities.iter() {
        if let Some(mut ec) = commands.get_entity(entity) {
            ec.insert(HealthBar::default());
        }
    }
}

fn despawn_unattached_healthbars(
    mut commands: Commands,
    healthbars: Query<(Entity, &HealthBarAttach), Without<HealthBar>>,
    entites: Query<(&Health, &Transform), With<HealthBar>>,
) {
    for (hb_entity, attach) in healthbars.iter() {
        // despawn the healthbar
        if let Err(_) = entites.get(attach.attached_to) {
            if let Some(ec) = commands.get_entity(hb_entity) {
                ec.despawn_recursive()
            }
        }
    }
}

fn update_healthbars(
    mut healthbars: Query<
        (
            Entity,
            &mut Text,
            &mut Style,
            &mut Transform,
            &HealthBarAttach,
            &mut Visibility,
        ),
        Without<HealthBar>,
    >,
    asset_server: Res<AssetServer>,
    entites: Query<(&Health, &Transform, &HealthBar)>,
    camera_q: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) {
    for (_hb_entity, mut hb_text, mut hb_style, mut hb_transform, hb_attach, mut hb_visibility) in
        healthbars.iter_mut()
    {
        if let Ok((e_health, e_transform, e_bar)) = entites.get(hb_attach.attached_to) {
            let bartrans = get_sceen_transform_and_visibility(&camera_q, e_transform);
            // *hb_transform = bartrans;
            *hb_visibility = Visibility::Visible;

            hb_style.position.left = Val::Px(bartrans.0);
            hb_style.position.bottom = Val::Px(bartrans.1);

            let current = e_health.current();
            let max = e_health.max();
            let style = TextStyle {
                font_size: e_bar.size,
                color: e_bar.color,
                font: asset_server.load("Rosela.ttf"),
            };

            *hb_text = Text {
                sections: [TextSection {
                    value: format!("{current}/{max}"),
                    style,
                    ..default()
                }]
                .to_vec(),
                ..default()
            };
        }
    }
}

fn spawn_health_bar_children(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    entities: Query<(Entity, &Health, &Transform, &HealthBar), Added<HealthBar>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) {
    for (entity, health, transform, hbar) in entities.iter() {
        let current = health.current();
        let max = health.max();
        let bartrans = get_sceen_transform_and_visibility(&camera_q, transform);

        commands.spawn(
            // Healthbarbundle
            HealthBarBundle {
                healthbar: HealthBarAttach {
                    attached_to: entity,
                },
                text: TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(bartrans.0),
                            top: Val::Px(bartrans.1),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("{current}/{max}"),
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

pub fn get_sceen_transform_and_visibility(
    camera_q: &Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
    transform: &Transform,
) -> (f32, f32) {
    let (camera, cam_gt) = camera_q.single();
    let pos = camera.world_to_viewport(cam_gt, transform.translation);
    if let Some(pos) = pos {
        (pos.x, pos.y)
    } else {
        (0.0, 0.0)
    }
}