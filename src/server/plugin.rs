use std::sync::mpsc::{Receiver, Sender};

use bevy::prelude::*;
use super::messages::{Messages, ClientMessages, ServerMessages};


#[derive(Resource)]
struct MessageReceiver(Receiver<ServerMessages>);

#[derive(Resource)]
struct MessageSender(Sender<ClientMessages>);

fn send_message_system(
    mut client_messages: EventReader<ClientMessages>
) {
    for message in client_messages.read() {

    }
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin{
   fn build(&self, app: &mut App) {


        


        app.add_event::<Messages>();

        // add the channel as a resource
        // implement the update system to add 
   } 
}
