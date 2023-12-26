
// Auras can either: 
// Impair movement 
// DoT/HoT
// Increase damage done/taken

use bevy::prelude::*;

enum Aura{
    MovementEffect,
    OvertimeEffect,
    DamageChange
}


/// OVERTIME
struct Overtime{
    damage_healing_flag: bool, // true -> healing, false -> damage
    amount: u32, 
    every: u32, 
    time_unit: OvertimeUnit
}

enum OvertimeUnit{
    Second, 
    Hour, 
    Minute,
}


#[derive(Component)]
struct DamageOrHealingOverTimeComponent{
    data: Overtime,
    timer: Timer
}


/// MOVEMENT EFFECT
struct MovementEffect{
    decrease_increase_flag: bool, // true -> increase, false -> decrease,
    percentage_change: u8 // value between 1-100 that determines how fast/slow this effect makes the target
}

/// DAMAGE INCREMENT/DECREMENT
struct DamageChange{
    amount: u32,
    value_type:  DamangeChangeType
}

enum DamangeChangeType {
    Percent, 
    Amount
}

fn emit_auras(

) {

}

pub struct AurasPlugin;

impl Plugin for AurasPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(emit_auras);
    }
}