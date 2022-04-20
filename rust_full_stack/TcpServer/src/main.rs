use std::net::TcpListener;
use std::io::{Read,Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9001").unwrap();
    for steam in listener.incoming(){
        let mut _steam = steam.unwrap();
        println!("Connection established!");

        let mut buffer = [0; 1024];
        _steam.read(&mut buffer).unwrap();
        _steam.write(&mut buffer);

    }
}
