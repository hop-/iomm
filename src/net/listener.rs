use crate::net::conn::Conn;

pub trait Listener {
    fn listen(port: u16) -> dyn Conn;
}