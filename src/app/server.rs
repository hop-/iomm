use crate::{
    app::connections::{
        Connection,
        Consumer,
        Producer,
    },
    error::TextError,
    net::{
        listener::Listener,
        conn::Conn,
        quic,
    },
};

use std::{
    error::Error,
};

use log::info;

pub struct Server {
    port: u16,
    //TODO: add members
}

impl Server {
    pub fn new(port: u16) -> Server {
        Server {
            port,
        }
    }

    #[tokio::main]
    pub async fn serve(&self) {
        info!("Starting server on port {}", self.port);
        let mut listener = quic::listener::Listener::new();

        loop {
            match listener.listen(self.port).await {
                Ok(conn) => {
                    info!("New Connection estblished");
                    // TODO: Do things with conn
                    self.handle_connection(conn).await.unwrap();
                },
                Err(err) => {
                    // TODO: handle errors
                    todo!()
                },
            }
        };
    }

    async fn handle_connection(&self, conn: Box<dyn Conn>) -> Result<(), Box<dyn Error>> {
        let conn= self.handshake(conn).await?;

        // TODO: handle conn
        //match conn {
        //    Connection::Consumer(c) => {
        //},
        //    Connection::Producer(p) => {
        //},
        //}

        Ok(())
    }

    async fn handshake(&self, mut conn: Box<dyn Conn>) -> Result<Connection, Box<dyn Error>> {
        let mut options_message = conn.recv().await?;
        let options = options_message.body_as_map();

        let c: Connection;

        // Defining connection type
        match options["type"].as_str() {
            "consumer" => {
                c = Connection::Consumer(Consumer::new(conn));
            },
            "producer" => {
                c = Connection::Producer(Producer::new(conn));
            },
            _ => return Err(Box::new(TextError::new("Recevied connection doen't fit requirements"))),
        }

        // TODO: use other options

        Ok(c)
    }
}
