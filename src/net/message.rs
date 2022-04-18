use std::{
    str,
    error::Error,
    collections::HashMap,
};
use bytes::Bytes;
use serde::{
    Serialize,
    Deserialize,
};

// Initially using json, can be changed
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    topic: String,
    body: Vec<u8>,

    #[serde(skip_serializing, skip_deserializing)]
    body_map: Option<HashMap<String, String>>,
}

impl Message {
    pub fn deserialize(message: &Bytes) -> Result<Message, Box<dyn Error>> {
        let message = serde_json::from_slice(&message[..])?;

        Ok(message)
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

    pub fn serialize(&self) -> Result<Bytes, Box<dyn Error>> {
        let message = serde_json::to_string(self)?;

        // TODO: find better way
        Ok(Bytes::copy_from_slice(message.as_bytes()))
    }
}