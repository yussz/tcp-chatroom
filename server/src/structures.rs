use::std::{
    sync::{Arc,Mutex},
    collections::HashMap,
    net::TcpStream
};

#[derive(Debug)]
pub struct User {
    pub username:String,
    pub stream:TcpStream
}


//scary lifetime
pub enum TypeRecievers<'a> {
    AllStreams(Arc<Mutex<HashMap<String,User>>>),
    Stream(&'a mut TcpStream)
}
