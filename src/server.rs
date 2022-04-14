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
        let listener = quic::listener::Listener::new();

        loop {
            let conn = listener.listen(self.port).await;
            // TODO:: Do things with conn
            // handle errors
            self.handle_connection(conn).await.unwrap();
        };
    }

    async fn handle_connection(&self, conn: Box<dyn Conn>) -> Result<(), Error> {
        // TODO: handle
        let conn_type = self.handshake(conn).await;

        Ok(())
    }

    async fn handshake(&self, conn: Box<dyn Conn>) -> Option<()> {
        let mut connection_option_message = conn.recv().await;

        for b in connection_option_message.body_as_map() {
        }

        None
    }
}
