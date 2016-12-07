use client::Client;

#[derive(Clone)]
pub struct Message {
    //TODO: Add source username
    //TODO: Add destination username
    message: String
}
impl Message {
    pub fn new(msg: String) -> Message {
        Message {
            message: msg
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.message.as_bytes()
    }

    pub fn to_string(&self) -> String {
        self.message.clone()
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

