use bevy::app::Plugin;

use super::model::{CastSpellInit, CastSpellFire};

pub struct SpellsPlugin;

impl Plugin for SpellsPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CastSpellInit>();
        app.add_event::<CastSpellFire>();
    }
}