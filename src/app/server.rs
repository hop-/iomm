use crate::{
    app::connections::{
        Connection,
        Consumer,
        Producer,
    },
    error::Error,
    net::{
        listener::Listener,
        conn::Conn,
        quic,
    },
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
                    // TODO: Do things with conn
                    self.handle_connection(conn).await.unwrap();
                },
                Err(err) => {
                    // TODO: handle errors
                },
            }
        };
    }

    async fn handle_connection(&self, conn: Box<dyn Conn>) -> Result<(), Error> {
        // TODO:: use better return
        let conn= self.handshake(conn).await;
        // TODO: handle

        Ok(())
    }

    async fn handshake(&self, conn: Box<dyn Conn>) -> Option<Connection> {
        let mut options_message = conn.recv().await;
        let options = options_message.body_as_map();

        let conn: Connection;

        // Defining connection type
        match options["type"].as_str() {
            "consumer" => conn = Connection::Consumer(Consumer::new()),
            "producer" => conn = Connection::Producer(Producer::new()),
            _ => return None
        }

        // TODO: use other options

        Some(conn)
    }
}
