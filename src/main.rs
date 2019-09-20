extern crate rayon;

use std::net::{TcpListener, TcpStream};
use std::io::{Write, Read};
use rayon::{ThreadPoolBuilder};

fn work(stream: &mut TcpStream) {
    let client_addr = stream.peer_addr().unwrap();
    println!("client addr: {}", client_addr);
    let mut buf = [0; 1024*1024];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                println!("EOF");
                break;
            },
            Ok(len) => {
                println!("Read {} bytes, echo back", len);
                stream.write_all(&buf[0..len]);
            },
            Err(err) => {
                println!("Error occurs: {:?}", err);
                break;
            }
        }
    }

    println!("Client closed");
}

fn main() {
    let thread_pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let mut stream: TcpStream = stream.unwrap();        
        thread_pool.install(|| work(&mut stream));
    }
}
