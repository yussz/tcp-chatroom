use std::{
    net::TcpStream,
    thread,
    sync::{Arc,Mutex}
};


mod utils;

use utils::handle_connection;

const PORT:&str = "9999";

fn main() {
    let stream = TcpStream::connect(format!("0.0.0.0:{}",PORT)).unwrap();
    handle_connection(stream);
}