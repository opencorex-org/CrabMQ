#![allow(dead_code)]
use crate::protocol::types::Publish;

#[derive(Default, Debug)]
pub struct Dispatcher;

impl Dispatcher {
    pub fn new() -> Self { Self }

    pub fn dispatch<'a>(&self, publish: &Publish, subscribers: impl Iterator<Item=&'a str>) {
        // Placeholder: in a real broker this would enqueue/send to clients
        for sub in subscribers { 
            log::debug!("dispatching to {} topic={} bytes={}", sub, publish.topic, publish.payload.len());
        }
    }
}
