use std::collections::VecDeque;

#[derive(Debug)]
pub struct InputManager {
    pub key_buffer: VecDeque<String>,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            key_buffer: VecDeque::new(),
        }
    }

    pub fn has_key(&mut self) -> bool {
        self.key_buffer.len() > 0
    }

    pub fn get_one_key(&mut self) -> Option<String> {
        self.key_buffer.pop_front()
    }

    pub fn add_key_to_buffer(&mut self, key: String) {
        self.key_buffer.push_back(key);
    }
}
