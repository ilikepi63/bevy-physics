use bevy::prelude::*;


#[derive(Event)]
struct MessageReceiveEvent{}


#[derive(Event)]
struct MessageSendEvent{}


pub fn run(port: u16) {
    App::new().add_plugins((MinimalPlugins));
}

// struct ServerPlugin;

// impl Plugin for ServerPlugin {
//     fn build(&self, app: &mut App) {}
// }
