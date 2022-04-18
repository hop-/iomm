use crate::{
    net,
    net::message::Message,
    error,
};

use async_trait::async_trait;
use quinn;
use std::{
    error::Error,
    net::SocketAddr,
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
    
    async fn send(&mut self, message: &Message) -> Result<(), Box<dyn Error>> {
        let message = message.serialize()?;
        self.send_stream.write_chunk(message).await?;

        Ok(())
    }
    
    async fn recv(&mut self) -> Result<Message, Box<dyn Error>> {
        let message = self.recv_stream.read_chunk(32 * 1024, false).await?;

        match message {
            Some(message) => {
                Ok(Message::deserialize(&message.bytes)?)
            },
            None => {
                Err(Box::new(error::Error {}))
            }
        }
    }

    fn close(&self) {
        // TODO: close
    }
    
    fn addr(&self) -> SocketAddr {
        self.conn.connection.remote_address()
    }
}