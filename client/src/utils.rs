use std::{
    io::{stdin, Read, Write}, 
    net::TcpStream, 
    thread
};
use colored::Colorize;


//call send prior to recieve since we want that colon out for user input, server will send msgs such as enter username and other client msgs.
pub fn send(mut stream:TcpStream) {
    let mut input = String::new();
    loop {
        stdin()
            .read_line(&mut input)
            .expect("error in getting your msg");

        stream.write_all(input.trim().as_bytes()).unwrap();
        input.clear();
    }
}

pub fn recieve(mut stream:TcpStream) {
    loop {
        let mut buffer :[u8;512]= [0;512];
        let usize = stream.read(&mut buffer).unwrap();
        if usize == 0 {
            println!("server closed");
            return
        }
        let msg = std::str::from_utf8(&buffer[..usize]).unwrap();
        println!("\t\t{}", msg.red());
    }
}   

pub fn handle_connection(stream: TcpStream) {
    //tcp streams allow cloning, so we can create a clone for read and write. wouldnt have been concurrent if used mutex.
    let stream_cl1 = stream.try_clone().unwrap();
    let send = thread::spawn(|| {
        send(stream_cl1);
    });

    let stream_cl2 = stream.try_clone().unwrap();
    let recieve = thread::spawn(|| {
        recieve(stream_cl2);
    });

    //continue after closure
    send.join().unwrap();
    recieve.join().unwrap();
}



