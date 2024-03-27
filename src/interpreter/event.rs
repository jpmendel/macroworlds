use crate::interpreter::state::object::{Line, Point, Size, TextStyle, TurtleShape};
use std::any::Any;
use std::collections::HashSet;
use std::error::Error;
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
    ObjectSize(Box<str>, Size),
    TurtleHeading(Box<str>, f32),
    TurtleShape(Box<str>, TurtleShape),
    TextPrint(Box<str>, String),
    TextClear(Box<str>),
    TextSize(Box<str>, f32),
    TextStyle(Box<str>, HashSet<TextStyle>),
    CanvasSize(f32, f32),
    BgColor(f32),
    BgPicture(String),
    PlacePicture(String, Point, Size),
    AddLine(Box<str>, Line),
    AddShape(Box<str>, String),
    Clean,
    ClearConsole,
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    Interrupt,
    KeyDown(String),
    KeyUp(String),
    Click(Point),
}

pub trait UiContext: Send + Sync {
    fn update_ui(&self);
    fn load_image(&self, name: Box<str>, path: String) -> Result<Box<dyn Any>, Box<dyn Error>>;
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
