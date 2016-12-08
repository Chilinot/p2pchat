extern crate argparse;
use argparse::*;

#[macro_use]
extern crate json;

mod actor_manager;
mod server;

mod client;

mod data;
use data::{Message, Data};

use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::Sender;
use std::thread;
use std::process;

fn main() {
    let mut username = String::new();
    let mut verbose = false;
    let mut client = true;
    let mut rhosts: Vec<String> = Vec::new();
    { // New scope for argument parser makes it simpler to reason about lifetimes.
        let mut parser = ArgumentParser::new();
        parser.set_description("P2P Chat system built in Rust as the final project for the LACPP-course.");

        parser.refer(&mut username)
            .add_argument("username", Store, "Username to use for the chat.")
            .required();

        parser.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Output lots of info.");

        parser.refer(&mut rhosts)
            .add_option(&["-r", "--remote"], List, "Define remote hosts.");

        parser.refer(&mut client)
            .add_option(&["--no-client"], StoreFalse,
                        "Disables the client part of the program. It will not connect to remote hosts.");

        parser.parse_args_or_exit();
    }

    // Start listening for connections.
    let acm_channel = server::bootup(verbose, username.clone());

    if client {
        println!("Client mode enabled.");

        if verbose {
            println!("Attempting to connect to supplied hosts...");
        }
        for rhost in rhosts.iter() {
            if verbose {
                println!("\tAttempting to connect to {}", &rhost);
            }
            connect(username.clone(), verbose, rhost, acm_channel.clone());
        }

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = match line {
                Ok(l) => l,
                Err(e) => {
                    println!("Could not read line from stdin! Error: {:?}", e);
                    process::exit(1);
                }
            };

            if line.starts_with("connect") {
                let line = line.trim_left_matches("connect ").to_string();
                connect(username.clone(), verbose, &line, acm_channel.clone());
            }
            else if line.starts_with("say") {
                let line = line.trim_left_matches("say").to_string();

                let msg = Data::Msg {
                    msg: Message::new(username.clone(), line, "general".to_string())
                };

                match acm_channel.send(msg) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Could not pass message to acm_channel! Error: {:?}", e);
                    }
                }
            }
            else if line.starts_with("quit") {
                process::exit(0);
            }
            else {
                println!("Error: unknown command!");
            }
        }
    }
    else {
        println!("Server mode enabled.");
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            if line.starts_with("quit") {
                process::exit(0);
            } else {
                println!("Unknown server command!");
            }
        }
    }
}

fn connect(username: String, verbose: bool, mut rhost: &String, acm: Sender<Data>) {

    if verbose {
        println!("Attempting to connect to {}", &rhost);
    }

    let addr = match SocketAddr::from_str(&mut rhost) {
        Ok(x) => x,
        Err(e) => {
            println!("Could not parse adress from input! {:?}", e);
            return;
        }
    };

    let mut socket = match TcpStream::connect(addr) {
        Ok(x) => x,
        Err(e) => {
            println!("Connection to {} refused! Error: {:?}", rhost, e);
            return;
        }
    };

    let msg = Message::new(username.clone(), String::new(), "handshake".to_string());

    match socket.write_all(msg.into_bytes().as_slice()) {
        Ok(_) => {
            match socket.flush() {
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

    let acm = acm.clone();
    thread::spawn(move || {
        server::handle_client(verbose, socket, acm, false, username.clone());
    });
}
