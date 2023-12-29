use std::time::Duration;

use bevy::utils::Uuid;

use crate::auras::Aura;


pub enum CastTime{
    Instant,
    Duration(Duration)
}  


/// Struct representing a spell initialization cast.
pub struct CastSpellInit{
    pub spell_name: String,
    pub cast_time: CastTime,
    pub damage: u32,
    pub apply_auras: Vec<Aura>
}

/// Event that represents the firing of a spell: 
/// This is the point where the spell becomes live and
/// is capable of doing it's primary stuff
pub struct CastSpellFire{
    pub id: String
}

