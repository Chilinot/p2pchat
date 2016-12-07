use std::sync::mpsc::Sender;
use std::net::IpAddr;
use data::*;

pub struct Client {
    //TODO: Chat history?
    ip_addr: IpAddr,
    username: String,
    channel: Sender<Data>
}

impl Client {
    pub fn new(ip: IpAddr, username: String, channel: Sender<Data>) -> Client {
        Client{
            ip_addr: ip,
            username: username,
            channel: channel
        }
    }

    pub fn send_message(&self, msg: Message) {
        self.channel.send(Data::Msg{msg:msg});
    }

    pub fn get_ip(&self) -> IpAddr {
        self.ip_addr.clone()
    }
}
