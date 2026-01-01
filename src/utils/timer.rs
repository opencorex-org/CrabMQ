#![allow(dead_code)]
use tokio::time::{self, Duration, Instant};

pub struct KeepAliveTimer {
    deadline: Instant,
}

impl KeepAliveTimer {
    pub fn new(seconds: u64) -> Self {
        Self { deadline: Instant::now() + Duration::from_secs(seconds) }
    }

    pub async fn wait(&self) {
        let now = Instant::now();
        if self.deadline > now {
            time::sleep(self.deadline - now).await;
        }
    }
}
