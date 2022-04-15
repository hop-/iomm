pub struct Consumer {}

impl Consumer {
    pub fn new() -> Consumer {
        Consumer {  }
    }
}

pub struct Producer {}

impl Producer {
    pub fn new() -> Producer {
        Producer {  }
    }
}

pub enum Connection {
    Consumer(Consumer),
    Producer(Producer),
}