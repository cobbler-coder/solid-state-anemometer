use super::Communication;
use std::collections::VecDeque;

pub struct MockComms {
    buffer: VecDeque<Vec<u8>>,
}

impl MockComms {
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
        }
    }
}

impl Communication for MockComms {
    fn send_message(&mut self, data: &[u8]) -> Result<(), String> {
        self.buffer.push_back(data.to_vec());
        Ok(())
    }

    fn read_message(&mut self) -> Result<Vec<u8>, String> {
        match self.buffer.pop_front() {
            Some(data) => Ok(data),
            None=> Err("No data in buffer".to_string()),
        }
    }
}