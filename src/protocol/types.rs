#![allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Publish {
    pub topic: String,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum Packet {
    Publish(Publish),
}
