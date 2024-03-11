use crate::state::object::{Line, TurtleShape};
use std::sync::mpsc;

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

pub struct EventHandler {
    pub ui_sender: mpsc::Sender<UiEvent>,
    pub input_receiver: mpsc::Receiver<InputEvent>,
}

impl EventHandler {
    pub fn from(
        ui_sender: mpsc::Sender<UiEvent>,
        input_receiver: mpsc::Receiver<InputEvent>,
    ) -> Self {
        EventHandler {
            ui_sender,
            input_receiver,
        }
    }

    pub fn send_ui_event(&self, event: UiEvent) {
        let _ = self.ui_sender.send(event);
    }
}
