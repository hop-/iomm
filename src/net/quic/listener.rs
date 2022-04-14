use crate::{
    net,
    net::quic::conn::Conn,
};
use async_trait::async_trait;

pub struct Listener {}

impl Listener {
    pub fn new() -> Listener {
        Listener {  }
    }
}

#[async_trait]
impl net::listener::Listener for Listener {
    async fn listen(&self, port: u16) -> Box<dyn net::conn::Conn> {
        // TODO: imlement listener with quic
        return Box::new(Conn::new());
    }
}