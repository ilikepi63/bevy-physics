// Auras can either:
// Impair movement
// DoT/HoT
// Increase damage done/taken

use bevy::{prelude::*};

use crate::{
    damage::{apply_damage, apply_health},
    health::Health,
};

pub enum Aura {
    MovementEffect,
    OvertimeEffect,
    DamageChange,
}

/// OVERTIME

#[derive(Component)]
pub struct OvertimeComponent {
    pub applied: Vec<Overtime>,
}

pub struct Overtime {
    pub damage_healing_flag: bool, // true -> healing, false -> damage
    pub amount: u32,
    pub every: u32,
    pub time_unit: OvertimeUnit,
    pub timer: Timer,
    pub count: u32,
}

impl Overtime {
    pub fn damage_per_second(amount: u32, count: u32) -> Self {
        Overtime {
            amount,
            damage_healing_flag: false,
            every: 1,
            time_unit: OvertimeUnit::Second,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            count,
        }
    }
}

pub enum OvertimeUnit {
    Second,
    Hour,
    Minute,
}

pub fn apply_overtime(
    entity: Entity,
    commands: &mut Commands,
    overtime: Overtime,
    applied: &mut Option<Mut<OvertimeComponent>>,
) {
    if let Some(comp) = applied {
        comp.applied.push(overtime);
    } else {
        let mut applied = Vec::with_capacity(100);

        applied.push(overtime);

        commands
            .entity(entity)
            .insert(OvertimeComponent { applied });
    }
}

pub fn remove_overtime(
    entity: Entity,
    commands: &mut Commands,
    overtime: Overtime,
    applied: &mut Option<OvertimeComponent>,
) {
    // let ent_commands = commands.entity(entity);

    if let Some(comp) = applied {
        comp.applied.push(overtime);
    } else {
        let mut applied = Vec::with_capacity(100);

        applied.push(overtime);

        commands
            .entity(entity)
            .insert(OvertimeComponent { applied });
    }
}

fn overtime_system(
    mut entities: Query<(Entity, &mut Health, &mut OvertimeComponent)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut health, mut overtime_comp) in entities.iter_mut() {
        for overtime in (*overtime_comp.applied).iter_mut() {
            if overtime.timer.just_finished() {
                if overtime.damage_healing_flag {
                    apply_health(&mut commands, entity, overtime.amount, &mut health)
                } else {
                    apply_damage(&mut commands, entity, overtime.amount, &mut health)
                }

                overtime.count -= 1;

                // if overtime.count < 1 {

                //     // add all of the depleted timings to the end
                //     overtime_comp.applied.swap(i, (overtime_comp.applied.len() - 1));
                //     remove_count += 1;
                // }
            }

            overtime.timer.tick(time.delta());
        }

        // removed depleted dps
        overtime_comp.applied.retain(|value| value.count > 0);

        // remove the buff if there are no overtimes working on the thing
        if overtime_comp.applied.is_empty() {
            if let Some(mut ent) = commands.get_entity(entity) { ent.remove::<OvertimeComponent>(); }
        };
    }
}

/// MOVEMENT EFFECT
struct MovementEffect {
    decrease_increase_flag: bool, // true -> increase, false -> decrease,
    percentage_change: u8, // value between 1-100 that determines how fast/slow this effect makes the target
}

/// DAMAGE INCREMENT/DECREMENT
struct DamageChange {
    amount: u32,
    value_type: DamangeChangeType,
}

enum DamangeChangeType {
    Percent,
    Amount,
}

// fn emit_auras() {}

pub struct AurasPlugin;

impl Plugin for AurasPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // app.add_system(emit_auras);
        app.add_systems(Update, overtime_system);
    }
}
