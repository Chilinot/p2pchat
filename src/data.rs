use client::Client;
use json::{parse, JsonValue, Error};

#[derive(Clone)]
pub struct Message {
    //TODO: Add destination username
    username: String,
    message: String,
    msg_type: String
}
impl Message {
    pub fn new(usr: String, msg: String, msg_type: String) -> Message {
        Message {
            username: usr,
            message: msg,
            msg_type: msg_type
        }
    }

    pub fn from_json(json: &String) -> Result<Message, Error> {
        let parsed = match parse(json.as_str()) {
            Ok(p) => p,
            Err(e) => {
                println!("Could not parse json!");
                return Err(e);
            }
        };

        Ok(Message {
            username: parsed["username"].as_str().unwrap().to_string(),
            message: parsed["message"].as_str().unwrap().to_string(),
            msg_type: parsed["type"].as_str().unwrap().to_string()
        })
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        let mut vector = self.to_json().dump().into_bytes();
        vector.push(0xA);
        return vector;
    }

    pub fn get_type(&self) -> String {
        self.msg_type.clone()
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    pub fn to_json(&self) -> JsonValue {
        object!{
            "username" => self.username.clone(),
            "message" => self.message.clone(),
            "type" => self.msg_type.clone()
        }
    }
}

pub enum Command {
    //TODO: More commands
    NewClient{client: Client},
    DeadClient{client: String}
}

pub enum Data {
    Msg{msg: Message},
    Cmd{cmd: Command}
}
