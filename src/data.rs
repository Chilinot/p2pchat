use std::net::SocketAddr;
use std::net::IpAddr;
use client::Client;

#[derive(Clone)]
pub struct Message {
    //TODO: Add source username
    //TODO: Add destination username
    source: SocketAddr,
    message: String
}
impl Message {
    pub fn new(src: SocketAddr, msg: String) -> Message {
        Message {
            source: src,
            message: msg
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.message.as_bytes()
    }

    pub fn to_string(&self) -> String {
        self.message.clone()
    }

    pub fn same_origin(&self, other: &IpAddr) -> bool {
        self.source.ip() == *other
    }
}

pub enum Command {
    //TODO: More commands
    NewClient{client: Client}
}

pub enum Data {
    Msg{msg: Message},
    Cmd{cmd: Command}
}

