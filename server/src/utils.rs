use crate::structures::{TypeRecievers,User};
use std::{
    io::{Read,Write},
    net::TcpStream,
    sync::{Arc,Mutex,mpsc},
    collections::HashMap
};

pub fn send(
    reciever_s:TypeRecievers,
    msg:Vec<u8>,
) {
    let buf = msg.as_slice();
    match reciever_s {
        //send msg to all users
        TypeRecievers::AllStreams(arr_of_users) => {
            let mut arr_of_users_lk = arr_of_users.lock().unwrap();
            for (_,user) in arr_of_users_lk.iter_mut() {
                user.stream.write_all(buf).unwrap();
            } 
        },
        //send msg to one user
        TypeRecievers::Stream(stream) => {
            stream.write_all(buf).unwrap();
        }
    }
    
}


pub fn recieve(stream: &mut TcpStream) -> Option<Vec<u8>> {
    let mut buffer:[u8;512] = [0;512];
    let usize = stream.read(&mut buffer).unwrap();
    if usize == 0 {
        return None
    }
    let msg =buffer[..usize].to_vec();
    Some(msg)
}   


pub fn handle_client(
    mut stream:TcpStream,
    arr_of_users:Arc<Mutex<HashMap<String,User>>>,
    tx:mpsc::Sender<Vec<u8>>
) {
    //get user's information and store in struct that will own all values.
    let user_ip = stream.peer_addr().unwrap().to_string();
    let initialize_msg= "enter your username".as_bytes().to_vec();
    send(TypeRecievers::Stream(&mut stream),initialize_msg);
    let username_bytes = recieve(&mut stream);
    if let Some(username_bytes) = username_bytes {
        //get the user's info and store in hashmap with a clone of stream that allows sending and recieving for the universal sending thread.
        let user = User {
            username:String::from_utf8(username_bytes.clone()).unwrap(),
            stream:stream.try_clone().unwrap()
        };
        let mut arr_of_users_lk  = arr_of_users.lock().unwrap();
        arr_of_users_lk.insert(stream.peer_addr().unwrap().to_string(),user);
        //unneeded but like explicity it's like free()
        std::mem::drop(arr_of_users_lk);
    }else {
        println!("{} disconnected without entering a username.",stream.peer_addr().unwrap().to_string());
        //no point of keeping the thread alive.
        return
    }
    
    let mut arr_of_users_lk = arr_of_users.lock().unwrap();
    //have to clone because the mutex will be locked and won't be able to access from universal sending thread. unfortunate, maybe structured this wrong.
    let user_handle = format!("{}:", arr_of_users_lk.get(&user_ip).unwrap().username.clone());
    std::mem::drop(arr_of_users_lk);
    loop {
        match recieve(&mut stream) {
            Some(msg) => {
                // pretty not efficient but cba to figure out how to append the bytes together without making clones.
                let msg = format!("{} {}", &user_handle,String::from_utf8(msg).unwrap()).into_bytes();
                tx.send(msg).unwrap()
            },
            None => {
                let mut arr_of_users_lk  = arr_of_users.lock().unwrap();
                arr_of_users_lk.remove(&stream.peer_addr().unwrap().to_string());
                println!("{}has left the chat.",&user_handle);
                return
            }
        }
    }

}
