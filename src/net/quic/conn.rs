use crate::{
    net,
    net::message::Message,
};
use async_trait::async_trait;
use quinn;
use std::{
    error::Error,
    net::SocketAddr
};

pub struct Conn {
    conn: quinn::NewConnection,
}

impl Conn {
    pub fn new(conn: quinn::NewConnection ) -> Conn {
        Conn {conn: conn}
    }
}

#[async_trait]
impl net::conn::Conn for Conn {
    
    async fn send(&self, msg: &Message) -> Result<(), Box<dyn Error>> {
        // TODO: implement send
        todo!()
    }
    
    async fn recv(&self) -> Result<Message, Box<dyn Error>> {
        // TODO: implement recieve
        todo!()
    }

    fn close(&self) {
        // TODO: close
    }
    
    fn addr(&self) -> SocketAddr {
        self.conn.connection.remote_address()
    }
}