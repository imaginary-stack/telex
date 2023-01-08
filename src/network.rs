pub const PORT: u16 = 9000;

use std::net::UdpSocket;

pub fn get_local_ipaddr() -> String {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("172.16.0.1:0").unwrap();
    match socket.local_addr() {
        Ok(addr) => return addr.ip().to_string() + ":" + &PORT.to_string(),
        _ => {
            eprintln!("Failed to find a valid network adapter");
            panic!()
        }
    }
}

pub mod receive {

    use std::{
        fmt::Error,
        io::Read,
        net::{TcpListener, TcpStream},
    };

    use crate::message::Message;

    use super::get_local_ipaddr;

    pub fn listener() {
        let listener = TcpListener::bind(get_local_ipaddr()).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(msg) => {
                    let msg = read_string(msg).unwrap();
                    let data: Message = Message::to_struct(msg);
                    println!("{}: {}", data.from, data.content);
                }
                Err(_) => panic!(),
            }
        }
    }

    fn read_string(mut msg: TcpStream) -> Result<String, Error> {
        let mut buffer = Vec::new();
        msg.read_to_end(&mut buffer).unwrap();
        let message = String::from_utf8(buffer.to_vec()).unwrap();
        return Ok(message);
    }
}

pub mod send {
    use std::{io::Write, net::TcpStream};

    use crate::message::Message;

    pub fn send() {
        let binding = Message::get_messages();
        let msg = Message::new(binding);
        let mut stream = TcpStream::connect("10.1.1.16:9000").unwrap();
        stream.write_all(msg.to_json().as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::{network::get_local_ipaddr, network::receive::listener};

    use super::send::send;

    #[test]
    fn test_get_local_ipaddr() {
        println!("{:?}", get_local_ipaddr());
    }

    #[test]
    fn test_read_string() {
        println!("{:?}", listener());
    }

    #[test]
    fn test() {
        send()
    }
}
