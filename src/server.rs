use crate::net::{
    listener::Listener,
    conn::Conn,
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
        // TODO: serve
        //let listener: dyn Listener = <SomeListener>::new();
        //while let Some(conn) = listener.listen(self.port).await {
        //    // TODO:: Do things with conn
        //    self.handle_connection(conn).unwrap()
        //};
    }

    async fn handle_connection(&self, conn: &dyn Conn) -> Result<(), Error> {
        // TODO: do handling
        let conn_type = self.handshake(conn).await;

        Ok(())
    }

    async fn handshake(&self, conn: &dyn Conn) -> Option<()> {
        let mut connection_option_message = conn.recv().await;

        for b in connection_option_message.body_as_map() {
        }

        None
    }
}
