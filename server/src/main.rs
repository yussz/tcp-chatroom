use std:: {
    collections::HashMap, 
    net::TcpListener, 
    sync::{mpsc, Arc, Mutex}, 
    thread
};


mod utils;
mod structures;

use utils::handle_client;
use utils::send;
use structures::{TypeRecievers,User};

const PORT :&str = "9999";

fn main()-> Result<(),&'static str> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}",PORT)).unwrap();
    
    //same memory shared across different threads
    let arr_of_users = Arc::new(Mutex::new(HashMap::<String,User>::new()));

    //single thread that will handle sending each client's msgs to all other clients.
    let (tx, rx) = mpsc::channel();

    //do not understand why you have to create a clone of the users in order to make a clone in the universal thread that handles sending msgs.
    let arr_of_users_cl = Arc::clone(&arr_of_users);
    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(msg) => {
                    let arr_of_users_cl = Arc::clone(&arr_of_users_cl);
                    send(TypeRecievers::AllStreams(arr_of_users_cl), msg);
                },
                Err(_)=> {
                    println!("error in recieving msg from thread in uni thread");
                }
            }    
        }
    });


    //get each connection and pass onto a different thread.
    for stream in listener.incoming() {
        
        match stream {
            Ok(stream) => {
                let tx_cl = mpsc::Sender::clone(&tx);
                let arr_of_users_cl = Arc::clone(&arr_of_users);
                thread::spawn(move|| {
                    handle_client(stream,arr_of_users_cl, tx_cl);
                });
            },
            Err(_) => {
                "error occured in initializing stream connection.";
            }
        }
    };


    Ok(())
}
  
