use super::movement::Movement;
use bevy::prelude::*;


#[derive(Event)]
pub enum ClientMessages {
    Movement(Movement),
}

#[derive(Event)]
pub enum ServerMessages {
    Movement(Movement),
}
