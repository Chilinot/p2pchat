use std::sync::mpsc::{Sender, channel};
use std::thread;
use client::Client;
use data::*;

pub struct ActorManager {
    client_list: Vec<Client>,
    verbose: bool
}

impl ActorManager {
    pub fn new(verbose: bool) -> ActorManager {
        ActorManager{
            client_list: vec![],
            verbose: verbose
        }
    }

    pub fn add_client(&mut self, c: Client) {
        self.client_list.push(c);
    }

    pub fn remove_client(&mut self, c: &str) {
        if self.verbose {
            println!("ActorManager removing deadclient {}", c);
        }
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
            else if self.verbose {
                println!("Didn't send message to sender!");
            }
        }
    }
}

pub fn spawn_actor_manager(verbose: bool) -> Sender<Data> {
    let (sender, receiver) = channel::<Data>();

    thread::spawn(move || {
        let mut actor_manager = ActorManager::new(verbose);
        loop {
            match receiver.recv() {
                Ok(data) => {
                    match data {
                        Data::Msg{msg} => {
                            actor_manager.broadcast(msg);
                        },
                        Data::Cmd{cmd} => {
                            match cmd {
                                Command::NewClient{client} => {
                                    actor_manager.add_client(client);
                                },
                                Command::DeadClient{client} => {
                                    actor_manager.remove_client(&client);
                                }
                            }
                        }
                    }
                },
                Err(e) => {
                    println!("Something went wrong when reading incoming data to actor manager! {:?}", e);
                }
            }
        }
    });

    return sender;
}
