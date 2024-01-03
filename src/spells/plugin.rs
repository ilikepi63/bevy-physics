use bevy::app::Plugin;

use super::{model::{CastSpellInit, CastSpellFire}, spell_system, casting::casting_system, spell_init_system};

pub struct SpellsPlugin;

impl Plugin for SpellsPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CastSpellInit>();
        app.add_event::<CastSpellFire>();
        app.add_system(spell_system);
        app.add_system(casting_system);
        app.add_system(spell_init_system);
    }
}