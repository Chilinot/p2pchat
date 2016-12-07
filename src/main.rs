extern crate argparse;
use argparse::*;

mod actor_manager;
mod server;

mod client;
use client::Client;

mod data;
use data::{Message, Data};

use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::Sender;
use std::thread;

fn main() {
    let mut verbose = false;
    let mut server = true;
    let mut client = true;
    let mut rhosts: Vec<String> = Vec::new();
    { // New scope for argument parser makes it simpler to reason about lifetimes.
        let mut parser = ArgumentParser::new();
        parser.set_description("P2P Chat system built in Rust as the final project for the LACPP-course.");

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
    let mut acm_channel = server::bootup();

    let this_addr = SocketAddr::from_str("127.0.0.1:8888");

    if client {
        println!("Running in client mode.");
        for mut rhost in rhosts.iter() {
            connect(rhost, acm_channel.clone());
        }

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap();

            if line.starts_with("connect") {
                let split: Vec<&str> = line.split(" ").collect();
                let rhost = split[1].to_string();
                connect(&rhost, acm_channel.clone());
            } else {
                let msg = Data::Msg {
                    msg: Message::new(this_addr.clone().unwrap(), line)
                };
                acm_channel.send(msg).unwrap();
            }

        }
    } else {
        println!("Server mode enabled.");
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let line = line.unwrap();
            if line == "terminate" {
                return;
            } else {
                println!("Unknown server command!");
            }
        }
    }
}

fn connect(mut rhost: &String, mut acm: Sender<Data>) {
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
            println!("Connection to {} refused!", rhost);
            return;
        }
    };

    let acm = acm.clone();
    thread::spawn(move || {
        server::handle_client(socket, acm, false);
    });
}
