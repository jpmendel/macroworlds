use crate::state::object::Line;

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
    TextText(String, String),
    TextSize(String, f32),
    AddLine(Line),
    Clean,
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    Interrupt,
    Key(String),
}
