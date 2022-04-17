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
    send_stream: quinn::SendStream,
    recv_stream: quinn::RecvStream,
}

impl Conn {
    pub fn new(conn: quinn::NewConnection, (send, recv): (quinn::SendStream, quinn::RecvStream) ) -> Conn {
        Conn {
            conn: conn,
            send_stream: send,
            recv_stream: recv,
        }
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