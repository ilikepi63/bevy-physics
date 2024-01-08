use std::time::Duration;

use bevy::ecs::event::Event;

use crate::auras::Aura;

#[derive(Event)]
pub enum CastTime {
    Instant,
    Duration(Duration),
}

/// Struct representing a spell initialization cast.
#[derive(Event)]
pub struct CastSpellInit {
    pub spell_id: String,
    pub cast_time: CastTime,
    pub damage: u32,
    pub apply_auras: Vec<Aura>,
}

/// Event that represents the firing of a spell:
/// This is the point where the spell becomes live and
/// is capable of doing it's primary stuff
#[derive(Event)]
pub struct CastSpellFire {
    pub id: String,
}
