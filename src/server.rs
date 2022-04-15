use crate::net::{
    listener::Listener,
    conn::Conn,
    quic,
};

use crate::error::Error;

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
        // TODO: handle
        let conn_type = self.handshake(conn).await;

        Ok(())
    }

    async fn handshake(&self, conn: Box<dyn Conn>) -> Option<String> {
        let mut options_message = conn.recv().await;
        let options = options_message.body_as_map();
        // TODO: use enum?
        let connection_type: String;

        match options["type"].as_str() {
            "consumer" => connection_type = "consumer".to_string(),
            "producer" => connection_type = "producer".to_string(),
            _ => return None
        }

        // TODO: use other options

        Some(connection_type)
    }
}
