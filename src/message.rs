use std::io;

use crate::network::get_local_ipaddr;
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub from: String,
    pub content: String,
    pub time: String,
    pub size: usize,
}

impl Message {
    pub fn new(content: String) -> Self {
        Message {
            from: get_local_ipaddr(),
            content: content.clone(),
            time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            size: content.as_bytes().len(),
        }
    }

    pub fn get_messages() -> String {
        let mut message = String::new();
        io::stdin().read_line(&mut message).unwrap();
        message
    }

    pub fn to_json(&self) -> String {
        let data = serde_json::to_string_pretty(&self).unwrap();
        data
    }

    pub fn to_struct(s: String) -> Self {
        let data = serde_json::from_str(&s).unwrap();
        data
    }
}

#[cfg(test)]
mod test {

    use std::env;

    use super::*;
    // static M: Message = Message::get_message("Hello World".to_string());
    #[test]
    fn test_new() {
        let message = Message::new("Hello World".to_string());

        println!("{:?}", message);
    }

    #[test]
    fn test_to_json() {
        let msg = Message::new("Hello World".to_string());
        let data = msg.to_json();
        println!("{}", data);
    }

    #[test]
    fn test() {
        let hostname = env::var("HOSTNAME").unwrap();
        println!("The hostname is: {}", hostname);
    }
}
