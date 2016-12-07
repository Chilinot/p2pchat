use client::Client;
use actor_manager::ActorManager;
use data::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpStream, TcpListener};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

pub fn bootup(verbose: bool) -> Sender<Data> {
    if verbose {
        println!("Setting up actor manager...");
    }
    let (ac_sender, ac_receiver) = channel::<Data>();
    thread::spawn(move || {
        let mut actor_manager = ActorManager::new();
        loop {
            match ac_receiver.recv() {
                Ok(data) => {
                    match data {
                        Data::Msg{msg} => {
                            actor_manager.broadcast(msg);
                        },
                        Data::Cmd{cmd} => {
                            match cmd {
                                Command::NewClient{client} => {
                                    actor_manager.add_client(client);
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

    if verbose {
        println!("Setting up listener actor...");
    }
    let acs = ac_sender.clone();
    thread::spawn(move || {
        //TODO: TcpListener
        let listener = TcpListener::bind("0.0.0.0:8888").unwrap();
        if verbose {
            println!("Listening for connections.");
        }
        for inc_stream in listener.incoming() {
            match inc_stream {
                Ok(mut stream) => {
                    // Successfull new connection.
                    // Spawn new thread and pass it a channel to the actor manager.
                    if verbose {
                        println!("New connection!");
                        println!("Asking connection for username...");
                    }
                    stream.write_all("Please send username.\n".as_bytes());
                    stream.flush();
                    let acs_clone = acs.clone();
                    thread::spawn(move || {
                        handle_client(verbose, stream, acs_clone, true);
                    });
                }
                Err(e) => {
                    println!("Client failed to connect: {:?}", e);
                }
            }
        }
    });

    return ac_sender.clone();
}

pub fn handle_client(verbose: bool, mut stream: TcpStream, acm: Sender<Data>, wait_for_name: bool) {
    // Create two buffered wrappers around the stream, one for the reader thread
    // and one for the writer thread.
    let mut buf_stream = BufReader::new(stream.try_clone().unwrap());

    let mut username = stream.peer_addr().unwrap().to_string();
    if wait_for_name {
        username.clear();
        match buf_stream.read_line(&mut username) {
            Ok(_) => {
                println!("New client connected with username: {}", &username);
            },
            Err(e) => {
                println!("Could not read username from socket! {:?}", e);
                return;
            }
        }
    }

    // Sender channel will be passed to ActorManager in a Client object.
    // Receiver channel will be passed to stream_writer()
    let (sender, receiver) = channel::<Data>();

    // Create new client object and send it to the actor manager
    let client = Client::new(stream.peer_addr().unwrap().ip(), username, sender);
    let msg = Data::Cmd {
        cmd: Command::NewClient {
            client: client
        }
    };
    acm.send(msg);

    // Reader thread
    thread::spawn(move || {
        stream_reader(acm, buf_stream.into_inner());
    });

    // Use this thread for the writer
    stream_writer(receiver, stream);
}

fn stream_reader(acm: Sender<Data>, mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    let mut buf_stream = BufReader::new(stream);
    let mut message = String::new();
    loop {
        message.clear();
        match buf_stream.read_line(&mut message) {
            Ok(_) => {
                println!("Remote says: {}", &message);
                // Send incoming message to actor manager
                let msg = Data::Msg { msg: Message::new(peer_addr.clone(), message.clone()) };
                acm.send(msg);
            },
            Err(e) => {
                println!("Could not read string from bufstream! {:?}", e);
                return;
            }
        }
    }
}

fn stream_writer(receiver: Receiver<Data>, mut stream: TcpStream) {
    // Loop indefinetly and read incoming messages
    loop {
        match receiver.recv() {
            Ok(data) => {
                match data {
                    Data::Msg{msg} => {
                        // Send message to client on other side of socket
                        stream.write_all(msg.as_bytes());
                        stream.flush();
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
