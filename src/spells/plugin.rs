use bevy::app::{Plugin, Update};

use super::{
    casting::casting_system,
    model::{CastSpellFire, CastSpellInit},
    spell_init_system, spell_system,
};

pub struct SpellsPlugin;

impl Plugin for SpellsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CastSpellInit>();
        app.add_event::<CastSpellFire>();
        app.add_systems(Update, (spell_system, casting_system, spell_init_system));
    }
}
