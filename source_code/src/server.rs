use client::Client;
use data::*;
use actor_manager;

use json;

use termion::color;

use std::io::prelude::*;
use std::io::BufReader;
use std::net::{SocketAddr, IpAddr, Ipv4Addr, TcpStream, TcpListener};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::process;
use std::collections::HashSet;

pub fn bootup(verbose: bool, username: String, port: u16) -> Sender<Data> {
    if verbose {
        println!("Setting up actor manager...");
    }
    let ac_sender = actor_manager::spawn_actor_manager(verbose);

    if verbose {
        println!("Setting up listener actor...");
    }
    let acs = ac_sender.clone();
    thread::spawn(move || {
        let local_addr = IpAddr::V4(Ipv4Addr::new(0,0,0,0));
        let local_sock_addr = SocketAddr::new(local_addr, port);
        let listener = match TcpListener::bind(local_sock_addr) {
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
                Ok(stream) => {
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

pub fn handle_client(
    verbose: bool,
    mut stream: TcpStream,
    acm: Sender<Data>,
    answer_handshake: bool,
    username: String
) {
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
    let client = Client::new(usr, sender);
    let msg = Data::Cmd {
        cmd: Command::NewClient {
            client: client
        }
    };
    match acm.send(msg) {
        Ok(_) => println!("User {} connected!", client_username.clone()),
        Err(e) => panic!("handle client could not send new client to actor manager! Error: {:?}", e)
    }

    // Reader thread
    thread::spawn(move || {
        stream_reader(client_username, verbose, acm, buf_stream.into_inner());
    });

    // Use this thread for the writer
    stream_writer(receiver, stream);
}

fn stream_reader(client_username: String, verbose: bool, acm: Sender<Data>, stream: TcpStream) {
    // I will use this hashmap as a hashset
    let mut received_messages = HashSet::new();

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
                    println!("{}{} sent empty string! Did the connection die? Killing connection just to be safe.{}", color::Fg(color::LightRed), &client_username, color::Fg(color::Reset));
                    let data = Data::Cmd{cmd: Command::DeadClient{client: client_username.clone()}};
                    match acm.send(data) {
                        Ok(_) => (),
                        Err(e) => {
                            panic!("Stream reader could not send dead client to actor manager! Error: {:?}", e);
                        }
                    }
                    return;
                }

                let msg = match Message::from_json(&message) {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Could not parse incoming message! Error: {:?}\n\tMessage: {:?}", e, &message);
                        continue;
                    }
                };

                match msg.get_type().as_ref() {
                    "general" => {
                        if !received_messages.contains(&message) {
                            println!("{}: {}", &msg.get_username(), msg.get_message());

                            let data = Data::Msg{msg: msg};
                            match acm.send(data) {
                                Ok(_) => (),
                                Err(e) => {
                                    panic!("Stream reader could not send new message to actor manager! Error: {:?}", e);
                                }
                            }

                            received_messages.insert(message.clone());
                        }
                    },

                    "bye" => {
                        // Notify actor manager that this client is now to be considered dead
                        let data = Data::Cmd{cmd: Command::DeadClient{client: client_username.clone()}};
                        match acm.send(data) {
                            Ok(_) => (),
                            Err(e) => {
                                panic!("Stream reader could not send new bye-message to actor manager! Error: {:?}", e);
                            }
                        }
                    },

                    // Ignore all handshakes at this stage
                    "handshake" => {
                        ();
                    }

                    _ => {
                        println!("Stream reader received message of unknown type!");
                    }
                }
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
                        match stream.write_all(msg.into_bytes().as_slice()) {
                            Ok(_) => {
                                match stream.flush() {
                                    Ok(_) => (),
                                    Err(e) => {
                                        println!("Stream writer could not flush socket! Error: {:?}", e);
                                    }
                                }
                            },
                            Err(e) => {
                                println!("Stream writer could not write msg to socket! Error: {:?}", e);
                            }
                        }
                    },
                    _ => {
                        println!("Stream writer received data that was not a message!");
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
