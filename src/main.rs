mod actor_manager;
mod server;

mod client;
use client::Client;

mod data;
use data::*;

use std::io;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpStream, TcpListener};

fn main() {

    server::bootup();

    let stdin = io::stdin();

    //TODO: Ask for IP and port of remote client
//    let mut rhost = String::new();
//
//    match stdin.lock().read_line(&mut rhost) {
//        Ok(_) => (),
//        Err(e) => panic!("Could not read rhost from stdin! {:?}", e)
//    }
//
//    let addr = match SocketAddr::from_str(&mut rhost) {
//        Ok(x) => x,
//        Err(e) => panic!("Could not parse adress from input! {:?}", e)
//    };

    let mut socket = TcpStream::connect("localhost:8888").unwrap();

    //TODO: Print out incoming data from socket

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        socket.write_all((line + "\n").as_bytes());
        socket.flush();
    }
}
