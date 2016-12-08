use std::sync::mpsc::Sender;
use data::*;

pub struct Client {
    //TODO: Chat history?
    username: String,
    channel: Sender<Data>
}

impl Client {
    pub fn new(username: String, channel: Sender<Data>) -> Client {
        Client{
            username: username,
            channel: channel
        }
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn send_message(&self, msg: Message) {
        match self.channel.send(Data::Msg{msg:msg}) {
            Ok(_) => (),
            Err(e) => {
                println!("Client {} could not send message over channel! Error: {:?}", self.username, e);
            }
        }
    }
}
