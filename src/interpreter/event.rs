use crate::state::object::{Line, TurtleShape};
use std::sync::{mpsc, Arc, Mutex};

#[derive(Debug, Clone)]
pub enum UiEvent {
    Done,
    Wait(u64),
    ConsolePrint(String),
    Announce(String),
    NewTurtle(String),
    NewText(String),
    RemoveObject(String),
    ObjectPos(String, (f32, f32)),
    ObjectColor(String, f32),
    ObjectVisible(String, bool),
    TurtleHeading(String, f32),
    TurtleShape(String, TurtleShape),
    TextPrint(String, String),
    TextClear(String),
    TextSize(String, f32),
    CanvasSize(f32, f32),
    BgColor(f32),
    AddLine(String, Line),
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
    pub input_receiver: mpsc::Receiver<InputEvent>,
}

impl EventHandler {
    pub fn new(input_receiver: mpsc::Receiver<InputEvent>) -> Self {
        EventHandler {
            ui_handler: None,
            ui_context: None,
            input_receiver,
        }
    }

    pub fn send_ui_event(&self, event: UiEvent) {
        if let Some(handler) = self.ui_handler.clone() {
            if let Some(context) = self.ui_context.clone() {
                let mut handler = handler.lock().unwrap();
                handler.handle_ui_event(context, event);
                return;
            }
        }
        println!("Event handler not configured");
    }

    pub fn receive_input_event(&self) -> Result<InputEvent, mpsc::TryRecvError> {
        self.input_receiver.try_recv()
    }
}
