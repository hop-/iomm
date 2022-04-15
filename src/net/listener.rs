use crate::net::conn::Conn;
use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Listener {
    async fn listen(&mut self, port: u16) -> Result<Box<dyn Conn>, Box<dyn Error>>;
}