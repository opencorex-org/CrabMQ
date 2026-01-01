#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
pub struct Router {
    // topic -> set of client_ids
    subs: HashMap<String, HashSet<String>>, 
}

impl Router {
    pub fn new() -> Self { Self::default() }

    pub fn subscribe(&mut self, client_id: &str, topic: &str) {
        self.subs.entry(topic.to_string()).or_default().insert(client_id.to_string());
    }

    pub fn unsubscribe(&mut self, client_id: &str, topic: &str) {
        if let Some(set) = self.subs.get_mut(topic) {
            set.remove(client_id);
            if set.is_empty() { self.subs.remove(topic); }
        }
    }

    pub fn subscribers(&self, topic: &str) -> impl Iterator<Item=&str> {
        self.subs.get(topic).into_iter().flat_map(|s| s.iter().map(|x| x.as_str()))
    }
}
