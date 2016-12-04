use std::net::{TcpStream, TcpListener};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

struct Client {
    username : String,
    socket : TcpStream
}

fn handle_client(stream: TcpStream, acm: Sender<String>) {
    // Allocate a 2KiB buffer
    let mut buf = [0u8; 2048];
}

fn main() {

    print!("Setting up actor manager...");
    let (ac_sender, ac_receiver) = channel::<String>();
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
                    let acs_clone = acs_clone.clone();
                    thread::spawn(move || {
                        handle_client(stream, acs_clone);
                    });
                }
                Err(e) => {
                    // Connection failed
                }
            }
        }
    });
}
