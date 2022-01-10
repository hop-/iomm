use std::collections::HashMap;

pub struct Message {
    topic: String,
    body: Vec<u8>,
    body_map: Option<HashMap<String, String>>,
}

impl Message {
    pub fn new(topic: String, body: Vec<u8>) -> Message {
        Message {
            topic: topic,
            body: body,
            body_map: None
        }
    }

    pub fn new_internal(topic: String, body: Vec<u8>) -> Message {
        Message::new(format!("internal:{}", topic), body)
    }

    pub fn topic(&self) -> &String {
        &self.topic
    }

    pub fn body(&self) -> &Vec<u8> {
        &self.body
    } 

    pub fn body_as_map(&mut self) -> &HashMap<String, String> {
        if self.body_map.is_some() {
            return self.body_map.as_ref().unwrap();
        }
        self.body_map = Some(HashMap::new());

        // TODO: parse body

        self.body_map.as_ref().unwrap()
    }
}