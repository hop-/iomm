use crate::{
    net,
    net::quic::conn::Conn,
};

use std::{
    sync::Arc,
    error::Error,
    net::{
        SocketAddr,
        Ipv4Addr,
        IpAddr,
    },
};

use quinn;
use async_trait::async_trait;
use futures_util::stream::StreamExt;

// Some tmp server config copied from quinn examples
fn configure_server() -> Result<(quinn::ServerConfig, Vec<u8>), Box<dyn Error>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()]).unwrap();
    let cert_der = cert.serialize_der().unwrap();
    let priv_key = cert.serialize_private_key_der();
    let priv_key = rustls::PrivateKey(priv_key);
    let cert_chain = vec![rustls::Certificate(cert_der.clone())];

    let mut server_config = quinn::ServerConfig::with_single_cert(cert_chain, priv_key)?;
    Arc::get_mut(&mut server_config.transport)
        .unwrap()
        .max_concurrent_uni_streams(0_u8.into());

    Ok((server_config, cert_der))
}
pub struct Listener {
    config: quinn::ServerConfig,
    endpoint: Option<quinn::Endpoint>,
    incoming: Option<quinn::Incoming>,
    port: u16
}

impl Listener {
    pub fn new() -> Listener {
        let (config, _) = configure_server().unwrap();

        Listener {
            config: config,
            endpoint: None,
            incoming: None,
            port: 0,
        }
    }
}

#[async_trait]
impl net::listener::Listener for Listener {
    async fn listen(&mut self, port: u16) -> Result<Box<dyn net::conn::Conn>, Box<dyn Error>> {
        // TODO: verify port?
        // TODO: maybe move to constroctor
        if port != self.port {
            let listen = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
            let (e, i) = quinn::Endpoint::server(self.config.clone(), listen)?;

            self.endpoint = Some(e);
            self.incoming = Some(i);
            self.port = port;
        }

        // TODO: maybe better to use "?"
        match self.incoming.as_mut().unwrap().next().await {
            Some(cing) => {
                let conn = cing.await?; 
                return Ok(Box::new(Conn::new(conn)));
            },
            None => {
                todo!()
            }
        }
        //match &self.incoming {
        //    Some(mut incoming) => {
        //        let conn = incoming.next().await;
        //        return Ok(Box::new(Conn::new()));
        //    },
        //    None => {
        //        // TODO: handle error?
        //        todo!()
        //    },
        //}
    }
}