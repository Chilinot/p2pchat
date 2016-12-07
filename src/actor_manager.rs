use client::Client;
use data::*;

pub struct ActorManager {
    client_list: Vec<Client>
}

impl ActorManager {
    pub fn new() -> ActorManager {
        ActorManager{
            client_list: vec![]
        }
    }

    pub fn add_client(&mut self, c: Client) {
        self.client_list.push(c);
    }

    //TODO: remove_client()

    pub fn broadcast(&self, msg: Message) {
        for client in self.client_list.iter() {
            // Pass each client a copy of the message.
            let msg = msg.clone();
            client.send_message(msg);
        }
    }
}
