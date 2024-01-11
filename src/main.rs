#![allow(clippy::type_complexity)]

use std::time::Duration;

use auras::AurasPlugin;
use bevy::app::{Startup, Update};
use bevy::ecs::event::EventWriter;

use bevy::pbr::AmbientLight;
use bevy::prelude::{
    App, Color, Commands, Component, Input, KeyCode, MouseButton, Query, Res, Resource, With,
};
use bevy::gizmos::gizmos::Gizmos;

use bevy::window::{CursorGrabMode, PrimaryWindow, Window};
use bevy::DefaultPlugins;
// use bevy_inspector_egui::egui::Key;

use bevy_xpbd_3d::plugins::{PhysicsDebugPlugin, PhysicsPlugins};
use character_controller::{create_character_controller, update_character_transform};

use damage_text::DamageTextPlugin;
use enemy::EnemyPlugin;
use fps_measure::{FpsMeasurePlugin, setup_fps_counter, fps_text_update_system};
use health::health_system;
use health_bars::HealthBarPlugin;
use lifetime::LifetimePlugin;
use map::setup_map;
use projectile::ProjectilePlugin;
use spells::{CastSpellInit, SpellsPlugin};
use ui::UIPlugin;
use bevy_mod_raycast::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

pub mod character_controller;
mod aoe;
mod auras;
mod controller;
mod damage;
mod damage_text;
pub mod enemy;
mod health;
pub mod health_bars;
pub mod hit_box;
mod lifetime;
mod map;
pub mod orbit_camera;
mod particles;
pub mod projectile;
mod spells;
mod ui;
pub mod utils;
mod server;
mod fps_measure;

pub const PLAYER: u16 = 0b1;
pub const STATIC_GEOMETRY: u16 = 0b10;

#[derive(Resource)]
pub struct MovementResource {
    pub speed: f32,
}

impl Default for MovementResource {
    fn default() -> Self {
        Self { speed: 7.0 }
    }
}

#[derive(Component)]
struct Despawnable {}

#[derive(Component)]
pub struct Floor {}

fn main() {
    App::new()
        .init_resource::<InputState>()
        .init_resource::<MovementResource>()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            controller::CharacterControllerPlugin,
            FrameTimeDiagnosticsPlugin::default(),
            orbit_camera::OrbitCameraPlugin,
            EnemyPlugin,
            LifetimePlugin,
            HealthBarPlugin,
            ProjectilePlugin,
            DamageTextPlugin,
            AurasPlugin,
            UIPlugin,
            SpellsPlugin,
            aoe::AoeTargetingPlugin,
            DefaultRaycastingPlugin
        ))
        .add_systems(
            Startup,
            (setup_map, setup_graphics, create_character_controller, setup_fps_counter),
        )
        // .add_startup_system(setup_world)
        .add_systems(
            Update,
            (
                cursor_grab,
                on_mouse_shoot,
                health_system,
                basic_attack,
                update_character_transform, // character_direction_system
                fps_text_update_system
                // raycast
            ),
        )
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // light

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
}



#[derive(Resource, Default)]
struct InputState {
    // reader_motion: ManualEventReader<MouseMotion>,
}

fn cursor_grab(
    buttons: Res<Input<MouseButton>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if buttons.just_pressed(MouseButton::Right) {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }

        if buttons.just_released(MouseButton::Right) {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    } else {
        // warn!("Primary window not found for `cursor_grab`!");
    }
}

fn on_mouse_shoot(
    buttons: Res<Input<KeyCode>>,
    mut spell_writer: EventWriter<spells::CastSpellInit>,
) {
    if buttons.just_pressed(KeyCode::Q) {
        spell_writer.send(CastSpellInit {
            spell_id: "s".to_string(),
            cast_time: spells::CastTime::Duration(Duration::from_secs(2)),
            damage: 10,
            apply_auras: vec![],
        });
    }
}

fn basic_attack(
    buttons: Res<Input<KeyCode>>,
    mut spell_writer: EventWriter<spells::CastSpellInit>,
) {
    if buttons.just_pressed(KeyCode::R) {
        spell_writer.send(CastSpellInit {
            spell_id: "a".to_string(),
            cast_time: spells::CastTime::Instant,
            damage: 10,
            apply_auras: vec![],
        });
    }
}

// fn display_events(
//     mut collision_events: EventReader<CollisionEvent>,
//     // _contact_force_events: EventReader<ContactForceEvent>,
//     mut commands: Commands,
//     mut health_entities: Query<(Entity, &mut Health)>,
//     damage_dealer_entities: Query<(Entity, &Damage)>,
// ) {
//     for collision_event in collision_events.iter() {
//         match collision_event {
//             CollisionEvent::Started(first_entity, second_entity, _) => {
//                 let mut health_ent = health_entities.get_mut(*first_entity).ok();

//                 if let None = health_ent {
//                     health_ent = health_entities.get_mut(*second_entity).ok();
//                 }

//                 let (hit_ent, mut health) = if let Some(ent) = health_ent {
//                     ent
//                 } else {
//                     // debug!("No entity with health!");
//                     return;
//                 };

//                 let damage_ent = damage_dealer_entities
//                     .get(*first_entity)
//                     .map_err(|_| damage_dealer_entities.get(*second_entity))
//                     .ok();

//                 if let Some((_, Damage { amount })) = damage_ent {
//                     info!("Applying damage!");

//                     commands
//                         .entity(hit_ent)
//                         .insert(AppliedDamage { value: *amount });

//                     // health.current = health.current - amount;
//                 };
//             }
//             _ => {}
//         }
//     }

//     // for contact_force_event in contact_force_events.iter() {
//     //     println!("Received contact force event: {contact_force_event:?}");
//     // }
// }
