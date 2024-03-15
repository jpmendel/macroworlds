use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
pub struct InputManager {
    key_buffer: VecDeque<String>,
    keys_down: HashSet<String>,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            key_buffer: VecDeque::new(),
            keys_down: HashSet::new(),
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

    pub fn is_key_down(&self, key: &String) -> bool {
        self.keys_down.contains(key)
    }

    pub fn set_key_down(&mut self, key: String) {
        self.keys_down.insert(key);
    }

    pub fn set_key_up(&mut self, key: &String) {
        self.keys_down.remove(key);
    }
}
