use crate::net::message::Message;
use async_trait::async_trait;
use std::{
    error::Error,
    net::SocketAddr
};

#[async_trait]
pub trait Conn {
    async fn send(&mut self, msg: &Message) -> Result<(), Box<dyn Error>>;
    async fn recv(&mut self) -> Result<Message, Box<dyn Error>>;

    fn close(&self);
    fn addr(&self) -> SocketAddr;
}
