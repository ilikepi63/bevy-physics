use bevy::prelude::*;
use std::time::Duration;

use crate::character_controller::Player;

use super::{CastSpellFire};

#[derive(Component)]
pub struct Casting {
    pub current_duration: Duration,
    pub total_duration: Duration,
    pub spell_id: String,
}

pub fn setup_cast_bar() {}

pub fn casting_system(
    // this should also be the player
    mut caster_query: Query<(Entity, &mut Casting), With<Player>>,
    time: Res<Time>,
    mut cast_spell_fire_events: EventWriter<CastSpellFire>,
    mut commands: Commands,
) {
    for (entity, mut casting) in &mut caster_query {
        casting.current_duration = time.delta() + casting.current_duration;

        if casting.current_duration > casting.total_duration {
            cast_spell_fire_events.send(CastSpellFire {
                id: casting.spell_id.clone(),
            });
            commands.entity(entity).remove::<Casting>();
        }
    }
}
