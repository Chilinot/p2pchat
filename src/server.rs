use client::Client;
use actor_manager::ActorManager;
use data::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpStream, TcpListener};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::process;
use json;

pub fn bootup(verbose: bool, username: String) -> Sender<Data> {
    if verbose {
        println!("Setting up actor manager...");
    }
    let (ac_sender, ac_receiver) = channel::<Data>();
    thread::spawn(move || {
        let mut actor_manager = ActorManager::new(verbose);
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

    if verbose {
        println!("Setting up listener actor...");
    }
    let acs = ac_sender.clone();
    thread::spawn(move || {
        let listener = match TcpListener::bind("0.0.0.0:8888") {
            Ok(l) => l,
            Err(e) => {
                println!("Could not bind serverport 8888! Error: {:?}", e);
                process::exit(1);
            }
        };

        if verbose {
            println!("Listening for connections.");
        }
        for inc_stream in listener.incoming() {
            match inc_stream {
                Ok(mut stream) => {
                    // Successfull new connection.
                    // Spawn new thread and pass it a channel to the actor manager.
                    let acs_clone = acs.clone();
                    let usr = username.clone();
                    thread::spawn(move || {
                        handle_client(verbose, stream, acs_clone, true, usr);
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

pub fn handle_client(verbose: bool, mut stream: TcpStream, acm: Sender<Data>, answer_handshake: bool, username: String) {
    // Create two buffered wrappers around the stream, one for the reader thread
    // and one for the writer thread.
    let mut buf_stream = BufReader::new(stream.try_clone().unwrap());

    if verbose {
        println!("handle_client triggered!");
    }

    // Listen for initial handshake
    let mut incoming = String::new();
    match buf_stream.read_line(&mut incoming) {
        Ok(_) => {
            if verbose {
                println!("New connection, handshake: {:?}", &incoming);
            }
        },
        Err(e) => {
            println!("Could not read handshake from socket! {:?}", e);
            return;
        }
    }
    let json = json::parse(&incoming).unwrap();
    let client_username = json["username"].as_str().unwrap().to_string();

    if answer_handshake {
        let msg = Message::new(username.clone(), String::new(), "handshake".to_string());
        match stream.write_all(msg.into_bytes().as_slice()) {
            Ok(_) => {
                match stream.flush() {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Could not flush socket after sending handshake! Error: {:?}", e);
                        return;
                    }
                }
            },
            Err(e) => {
                println!("Could not send handshake over socket! Error: {:?}", e);
                return;
            }
        }
    }

    // Sender channel will be passed to ActorManager in a Client object.
    // Receiver channel will be passed to stream_writer()
    let (sender, receiver) = channel::<Data>();

    // Create new client object and send it to the actor manager
    let usr = client_username.clone();
    let client = Client::new(stream.peer_addr().unwrap().ip(), usr, sender);
    let msg = Data::Cmd {
        cmd: Command::NewClient {
            client: client
        }
    };
    acm.send(msg);

    // Reader thread
    thread::spawn(move || {
        stream_reader(client_username, verbose, acm, buf_stream.into_inner());
    });

    // Use this thread for the writer
    stream_writer(receiver, stream);
}

fn stream_reader(client_username: String, verbose: bool, acm: Sender<Data>, mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    let mut buf_stream = BufReader::new(stream);
    let mut message = String::new();
    loop {
        message.clear();
        match buf_stream.read_line(&mut message) {
            Ok(_) => {
                if verbose {
                    println!("{} sent: {:?}", &client_username, &message);
                }
                if message == "" {
                    println!("{} sent empty string! Did the connection die? Killing connection just to be safe.", &client_username);
                    let data = Data::Cmd{cmd: Command::DeadClient{client: client_username.clone()}};
                    acm.send(data);
                    return;
                }
                // Send incoming message to actor manager
                let msg = match Message::from_json(&message) {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Could not parse incoming message! Error: {:?}\n\tMessage: {:?}", e, &message);
                        continue;
                    }
                };

                println!("{}: {}", &client_username, msg.get_message());

                let data = Data::Msg{msg: msg};
                acm.send(data);
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
                        stream.write_all(msg.into_bytes().as_slice());
                        stream.flush();
                    },
                    Data::Cmd{cmd} => {
                        panic!("Client receiver received command!");
                    }
                }
            },
            Err(e) => {
                println!("stream_writer could not receive from channel! {:?}", e);
                return;
            }
        }
    }
}
