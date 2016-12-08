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
    pub fn remove_client(&mut self, c: &str) {
        let mut index = 0_usize;
        for client in self.client_list.iter() {
            if client.get_username() ==  c {
                break;
            }
            else {
                index += 1;
            }
        }

        self.client_list.remove(index);
    }

    pub fn broadcast(&self, msg: Message) {
        for client in self.client_list.iter() {
            if client.get_username() != msg.get_username() {
                let msg = msg.clone();
                client.send_message(msg);
            }
            else {
                println!("Didn't send message to sender!");
            }
        }
    }
}
