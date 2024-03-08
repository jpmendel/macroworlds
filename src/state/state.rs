use crate::state::canvas::Canvas;
use crate::state::datastore::DataStore;
use crate::state::input::InputManager;

#[derive(Debug)]
pub struct State {
    pub data: DataStore,
    pub canvas: Canvas,
    pub input: InputManager,
}

impl State {
    pub fn new() -> Self {
        State {
            data: DataStore::new(),
            canvas: Canvas::new(),
            input: InputManager::new(),
        }
    }
}
