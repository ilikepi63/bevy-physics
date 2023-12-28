use std::time::Duration;

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