use std::thread;
use std::io::prelude::*;
use std::net::{TcpStream, SocketAddr, IpAddr};

const TCP_PORT: u16 = 5002;
const HOST: [u8; 4] = [127, 0, 0, 1];

fn create_thread_data(n: u8) -> String {
    let mut s = "".to_string();
    s.push_str("Thread number >>");
    s.push_str(&n.to_string());
    s
}

fn main() {
    let mut threads = Vec::new();
    for n in 0..4 {
        println!("Creating {}", n);
        threads.push(thread::spawn(move || {
            let thread_data = create_thread_data(n);
            let mut server_response = String::new();

            let mut stream = TcpStream::connect(SocketAddr::new(IpAddr::from(HOST), TCP_PORT)).unwrap();

            println!("Sending {}", thread_data);

            stream.write(thread_data.as_bytes()).unwrap();

            stream.read_to_string(&mut server_response).unwrap();

            println!("Reading {}", server_response);
        }));
    }

    threads
        .into_iter()
        .for_each(|handle| {
            handle.join().unwrap();
        });
}