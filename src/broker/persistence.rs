#![allow(dead_code)]
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Persistence {
    retained: HashMap<String, Vec<u8>>, // topic -> payload
}

impl Persistence {
    pub fn new() -> Self { Self::default() }

    pub fn retain(&mut self, topic: &str, payload: Vec<u8>) {
        self.retained.insert(topic.to_string(), payload);
    }

    pub fn retained(&self, topic: &str) -> Option<&[u8]> {
        self.retained.get(topic).map(|v| v.as_slice())
    }
}
