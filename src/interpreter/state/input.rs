use crate::interpreter::state::object::Point;
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
pub struct InputManager {
    key_buffer: VecDeque<String>,
    keys_down: HashSet<String>,
    click_buffer: VecDeque<Point>,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            key_buffer: VecDeque::new(),
            keys_down: HashSet::new(),
            click_buffer: VecDeque::new(),
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

    pub fn has_click(&mut self) -> bool {
        self.click_buffer.len() > 0
    }

    pub fn get_one_click(&mut self) -> Option<Point> {
        self.click_buffer.pop_front()
    }

    pub fn add_click_to_buffer(&mut self, click: Point) {
        self.click_buffer.push_back(click);
    }
}
