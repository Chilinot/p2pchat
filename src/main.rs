use std::net::{TcpStream, TcpListener};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

// Load modules
mod client;
mod actor_manager;

// Import objects from modules
use client::Client;
use actor_manager::ActorManager;

#[derive(Clone)]
pub struct Message {
    message: String
}

pub enum Command {
    //TODO: More commands
    NEW_CLIENT{client: Client}
}

pub enum Data {
    //TODO: Add destination to messages.
    //For now all messages are broadcasted.
    MSG{msg: Message},
    CMD{cmd: Command}
}

fn handle_client(socket: TcpStream, acm: Sender<Data>) {
    // Allocate a 2KiB buffer
    let mut buf = [0u8; 2048];

    // Infinite loop will read and write from/to socket
    loop {
        let count = socket.read(&mut buf);
        //TODO: Add error management
        let usename = String::from_utf8(&buf).unwrap();
    }
}

fn main() {

    print!("Setting up actor manager...");
    let (ac_sender, ac_receiver) = channel::<Data>();
    thread::spawn(move || {
        //TODO: Actor Manager
    });

    print!("Setting up listener actor...");
    let acs_clone = ac_sender.clone();
    thread::spawn(move || {
        //TODO: TcpListener
        let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
        for inc_stream in listener.incoming() {
            match inc_stream {
                Ok(stream) => {
                    // Successfull new connection.
                    // Spawn new thread and pass it a channel to the actor manager.
                    let acs_clone = acs_clone.clone();
                    thread::spawn(move || {
                        handle_client(stream, acs_clone);
                    });
                }
                Err(e) => {
                    println!("Client failed to connect: {:?}", e);
                }
            }
        }
    });
}
