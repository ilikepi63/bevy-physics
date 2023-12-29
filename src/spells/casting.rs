use std::time::Duration;
use bevy::prelude::*;

use crate::character_controller::Player;

use super::model::CastSpellInit;

#[derive(Component)]
pub struct Casting{
    total_time: Duration,
    timer: Timer
}

pub fn setup_cast_bar(){


}

pub fn casting_system(
    // this should also be the player
    caster: Query<&Casting, With<Player>>
) {

}

pub fn apply_casting_component(
    mut commands: Commands,
    mut events: EventReader<CastSpellInit>, 
    player: Query<Entity, With<Player>>
) {
    for event in events.iter() {

        let player = player.single();

        // commands.entity(player).insert(Casting{
        //     total_time: event.cast_time,
        //     timer: Timer::from_seconds(event.)
        // });

    }
} 


