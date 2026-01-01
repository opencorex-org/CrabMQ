#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
pub struct Session {
    pub client_id: String,
    pub subscriptions: HashSet<String>,
}

impl Session {
    pub fn new(client_id: impl Into<String>) -> Self {
        Self { client_id: client_id.into(), subscriptions: HashSet::new() }
    }
}

#[derive(Default, Debug)]
pub struct SessionStore {
    inner: HashMap<String, Session>,
}

impl SessionStore {
    pub fn get_or_create(&mut self, client_id: &str) -> &mut Session {
        self.inner.entry(client_id.to_string()).or_insert_with(|| Session::new(client_id))
    }
}
