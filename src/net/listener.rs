use crate::net::conn::Conn;
use async_trait::async_trait;

#[async_trait]
pub trait Listener {
    async fn listen(&self, port: u16) -> Box<dyn Conn>;
}