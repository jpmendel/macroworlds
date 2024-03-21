use crate::interpreter::state::canvas::CanvasState;
use crate::interpreter::state::datastore::DataStore;
use crate::interpreter::state::input::InputManager;
use std::error::Error;
use std::time::SystemTime;

#[derive(Debug)]
pub struct State {
    pub program_time: SystemTime,
    pub data: DataStore,
    pub canvas: CanvasState,
    pub input: InputManager,
}

impl State {
    pub fn new() -> Self {
        State {
            program_time: SystemTime::now(),
            data: DataStore::new(),
            canvas: CanvasState::new(),
            input: InputManager::new(),
        }
    }

    pub fn get_time(&self) -> Result<u64, Box<dyn Error>> {
        let seconds = self.program_time.elapsed()?.as_secs();
        Ok(seconds / 10)
    }

    pub fn reset_timer(&mut self) {
        self.program_time = SystemTime::now();
    }
}
