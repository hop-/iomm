use crate::{
    net,
    net::message::Message,
};
use async_trait::async_trait;

pub struct Conn {}

impl Conn {
    pub fn new() -> Conn {
        Conn {}
    }
}

#[async_trait]
impl net::conn::Conn for Conn {
    
    async fn send(&self, msg: &Message) {
        // TODO: implement send
    }
    
    async fn recv(&self) -> Message {
        // TODO: implement recieve
        Message::new("not-imlemented".to_string(), [].to_vec())
    }

    fn close(&self) {
        // TODO: close
    }
    
    fn get_ip(&self) -> String {
        // TODO: get_ip
        "not.implemented".to_string()
    }
}