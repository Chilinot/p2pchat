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

    pub fn send_message(&self, msg: Message) {
        self.channel.send(Data::Msg{msg:msg});
    }
}
