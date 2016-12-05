use std::sync::mpsc::Sender;

use Message;

pub struct Client {
    //TODO: Chat history?
    username: String,
    channel: Sender<Message>
}

impl Client {
    pub fn new(username: String, channel: Sender<Message>) -> Client {
        Client{
            username: username,
            channel: channel
        }
    }

    pub fn send_message(&self, msg: Message) {
        self.channel.send(msg);
    }
}
