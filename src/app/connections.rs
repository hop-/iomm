use crate::{
    net::{
        conn::Conn,
    }
};
pub struct Consumer {
    conn: Box<dyn Conn>
}

impl Consumer {
    pub fn new(conn: Box<dyn Conn>) -> Consumer {
        Consumer {
            conn: conn
        }
    }
}

pub struct Producer {
    conn: Box<dyn Conn>
}

impl Producer {
    pub fn new(conn: Box<dyn Conn>) -> Producer {
        Producer {
            conn: conn
        }
    }
}

pub enum Connection {
    Consumer(Consumer),
    Producer(Producer),
}