extern crate bufstream;

mod client;
mod actor_manager;

use client::Client;
use actor_manager::ActorManager;

use bufstream::BufStream;

use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

#[derive(Clone)]
pub struct Message {
    message: String
}

impl Message {
    pub fn as_bytes(&self) -> &[u8] {
        self.message.as_bytes()
    }
}

pub enum Command {
    //TODO: More commands
    NewClient{client: Client}
}

pub enum Data {
    //TODO: Add destination to messages.
    //For now all messages are broadcasted.
    Msg{msg: Message},
    Cmd{cmd: Command}
}

fn handle_client(stream: TcpStream, acm: Sender<Data>) {

    // Create two buffered wrappers around the stream, one for the reader thread
    // and one for the writer thread.
    let mut buf_stream_reader = BufStream::new(stream.try_clone().unwrap());
    let mut buf_stream_writer = BufStream::new(stream);

    let mut username = String::new();
    match buf_stream_reader.read_line(&mut username) {
        Ok(_) => (),
        Err(e) => {
            println!("Could not read username from socket! {:?}", e);
            return;
        }
    }

    // Sender channel will be passed to ActorManager in a Client object.
    // Receiver channel will be passed to stream_writer()
    let (sender, receiver) = channel::<Data>();

    // Create new client object and send it to the actor manager
    let client = Client::new(username, sender);
    let msg = Data::Cmd {
        cmd: Command::NewClient {
            client: client
        }
    };
    acm.send(msg);

    // Reader thread
    thread::spawn(move || {
        stream_reader(acm, buf_stream_reader);
    });

    // Use this thread for the writer
    stream_writer(receiver, buf_stream_writer);
}

fn stream_reader(sender: Sender<Data>, mut stream: BufStream<TcpStream>) {
    let mut message = String::new();
    loop {
        match stream.read_line(&mut message) {
            Ok(_) => {
                // Send incoming message to actor manager
                let msg = Data::Msg {
                    msg: Message{message: message.clone()}
                };
                sender.send(msg);
            },
            Err(e) => {
                println!("Could not read string from bufstream! {:?}", e);
                return;
            }
        }
    }
}

fn stream_writer(receiver: Receiver<Data>, mut stream: BufStream<TcpStream>) {
    // Loop indefinetly and read incoming messages
    loop {
        match receiver.recv() {
            Ok(data) => {
                match data {
                    Data::Msg{msg} => {
                        // Send message to client on other side of socket
                        stream.write_all(msg.as_bytes());
                    },
                    Data::Cmd{cmd} => {
                        panic!("Client receiver received command!");
                    }
                }
            },
            Err(e) => {
                panic!("stream_writer could not receive from channel! {:?}", e);
            }
        }
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
