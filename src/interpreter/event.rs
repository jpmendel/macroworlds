use crate::interpreter::state::object::{Line, Point, TurtleShape};
use std::sync::{mpsc, Arc, Mutex};

#[derive(Debug, Clone)]
pub enum UiEvent {
    Done,
    Wait(u64),
    ConsolePrint(String),
    Announce(String),
    NewTurtle(Box<str>),
    NewText(Box<str>),
    RemoveObject(Box<str>),
    ObjectPos(Box<str>, Point),
    ObjectColor(Box<str>, f32),
    ObjectVisible(Box<str>, bool),
    TurtleHeading(Box<str>, f32),
    TurtleSize(Box<str>, f32),
    TurtleShape(Box<str>, TurtleShape),
    TextPrint(Box<str>, String),
    TextClear(Box<str>),
    TextSize(Box<str>, f32),
    CanvasSize(f32, f32),
    BgColor(f32),
    AddLine(Box<str>, Line),
    Clean,
    ClearConsole,
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    Interrupt,
    Key(String),
}

pub trait UiContext: Send + Sync {
    fn update_ui(&self);
}

pub trait UiEventHandler: Send + Sync {
    fn handle_ui_event(&mut self, ctx: Arc<Mutex<dyn UiContext>>, event: UiEvent);
}

pub struct EventHandler {
    pub ui_handler: Option<Arc<Mutex<dyn UiEventHandler>>>,
    pub ui_context: Option<Arc<Mutex<dyn UiContext>>>,
    pub input_receiver: Option<mpsc::Receiver<InputEvent>>,
}

impl EventHandler {
    pub fn new() -> Self {
        EventHandler {
            ui_handler: None,
            ui_context: None,
            input_receiver: None,
        }
    }

    pub fn send_ui(&self, event: UiEvent) {
        if let Some(handler) = self.ui_handler.clone() {
            if let Some(context) = self.ui_context.clone() {
                let mut handler = handler.lock().unwrap();
                handler.handle_ui_event(context, event);
            }
        }
    }

    pub fn receive_input(&self) -> Result<InputEvent, mpsc::TryRecvError> {
        if let Some(receiver) = &self.input_receiver {
            return receiver.try_recv();
        }
        Err(mpsc::TryRecvError::Empty)
    }
}
