use crate::state::object::{Line, TurtleShape};

#[derive(Debug, Clone)]
pub enum UiEvent {
    Done,
    Wait(u64),
    Print(String),
    NewTurtle(String),
    NewText(String),
    RemoveObject(String),
    ObjectPos(String, (f32, f32)),
    ObjectColor(String, f32),
    ObjectVisible(String, bool),
    TurtleHeading(String, f32),
    TurtleShape(String, TurtleShape),
    TextAddText(String, String),
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
