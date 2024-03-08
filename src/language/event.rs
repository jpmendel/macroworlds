use crate::language::turtle::Line;

#[derive(Debug, Clone)]
pub enum UiEvent {
    Done,
    Wait(u64),
    Print(String),
    NewTurtle(String),
    TurtlePos(String, (f32, f32)),
    TurtleHeading(String, f32),
    TurtleColor(String, f32),
    TurtleVisible(String, bool),
    AddLine(Line),
    Clean,
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    Interrupt,
    Key(String),
}
