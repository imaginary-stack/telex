use std::{
    fmt::Error,
    io::Read,
    net::{TcpListener, TcpStream, UdpSocket},
};

const PORT: u16 = 9000;

pub fn listener() {
    let listener = TcpListener::bind(get_local_ipaddr()).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(msg) => println!("{:?}", read_string(msg).unwrap()),
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

#[test]
fn test_get_local_ipaddr() {
    // let stream = TcpListener::bind("266.6.6.6:23");
    println!("{:?}", get_local_ipaddr());
}

#[test]
fn test_read_string() {
    println!("{:?}", listener());
}
