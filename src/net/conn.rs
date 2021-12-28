use crate::net::message::Message;

use async_trait::async_trait;

#[async_trait]
pub trait Conn {
    async fn send(&self, msg: &Message);
    async fn recv(&self) -> Message;

    fn close(&self);
    fn get_ip(&self) -> String;
}
