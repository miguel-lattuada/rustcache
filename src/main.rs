use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, SocketAddr, IpAddr};

const TCP_PORT: u16 = 5002;
const HOST: [u8; 4] = [127, 0, 0, 1];

fn display(bytes: usize, from: String, formatted: String) {
    println!("Bytes from: {}", from);
    println!("Bytes length: {}", bytes);
    println!("Bytes utf-8:, {}", formatted);
}

fn connection_handler(stream: &mut TcpStream, addr: SocketAddr) {
    let mut collect = [0; 256];
    let bytes_length = stream.read(&mut collect).unwrap();
    let data = String::from_utf8(collect.to_vec()).unwrap();

    display(bytes_length, addr.to_string(), data);
    thread::sleep(Duration::from_millis(2500));

    stream.write(&mut collect);
}

fn error_handler(error: std::io::Error) {
    println!("Error: {}", error);
}

fn main() -> Result<(), std::string::FromUtf8Error> {
    let addr: SocketAddr = SocketAddr::new(IpAddr::from(HOST), TCP_PORT);
    let listener: TcpListener = TcpListener::bind(addr).unwrap();

    loop {
        match listener.accept() {
            Ok((mut stream, _addr)) => {
                thread::spawn(move || {
                    connection_handler(&mut stream, addr)
                });
            },
            Err(e) => error_handler(e)
        }
    }
}
