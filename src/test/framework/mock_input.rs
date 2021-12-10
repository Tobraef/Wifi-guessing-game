use crate::communication::Input;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

pub struct MockInput {
    que: Arc<Mutex<std::collections::VecDeque<String>>>,
    buffer: String
}

impl Input for MockInput {
    fn receive(&mut self) -> &str {
        self.buffer = self.que.lock().unwrap().pop_front().unwrap();
        &self.buffer
    }
}

impl MockInput {
    #[allow(dead_code)]
    pub fn new() -> MockInput {
        MockInput {
            que: Arc::new(Mutex::new(std::collections::VecDeque::new())),
            buffer: String::new()
        }
    }

    #[allow(dead_code)]
    pub fn mock_input(&mut self, text: &str, secs_delay: u64) {
        let arc = self.que.clone();
        let text = String::from(text);
        thread::spawn(move || {
            let mut guard = arc.lock().unwrap();
            thread::sleep(Duration::from_secs(secs_delay));
            guard.push_back(text);
        });
    }
}