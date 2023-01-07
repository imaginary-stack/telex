use crate::network::get_local_ipaddr;
use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message<'a> {
    from: Option<String>,
    content: &'a str,
    time: String,
    size: usize,
}

impl<'a> Message<'a> {
    pub fn get_message(content: &'a str) -> Self {
        Message {
            from: Some(get_local_ipaddr()),
            content: content,
            time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            size: content.as_bytes().len(),
        }
    }

    pub fn to_json(&self) -> String {
        let data = serde_json::to_string_pretty(&self).unwrap();
        data
    }

    pub fn to_struct(s: &'a str) -> Self {
        let data = serde_json::from_str(&s).unwrap();
        data
    }
}

#[cfg(test)]
mod test {

    use super::*;
    // static M: Message = Message::get_message("Hello World".to_string());
    #[test]
    fn test_get_message() {
        let message = Message::get_message("Hello World");

        println!("{:?}", message);
    }

    #[test]
    fn test_to_json() {
        let msg = Message::get_message("Hello World");
        let data = msg.to_json();
        println!("{}", data);
    }
}
