use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;



fn main() {
    let mut _steam = TcpStream::connect("localhost:9001").unwrap();

    _steam.write("Hello".as_bytes()).unwrap();
    let mut buffer = [0; 5];
    _steam.read(&mut buffer).unwrap();
    println!("Response from server:{:?}", str::from_utf8(&buffer).unwrap());
}
