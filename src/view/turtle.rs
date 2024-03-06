use eframe::egui::{Color32, Pos2};

#[derive(Debug, Clone)]
pub struct Turtle {
    pub pos: Pos2,
    pub heading: f32,
    pub color: Color32,
    pub is_visible: bool,
    pub is_drawing: bool,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub start: Pos2,
    pub end: Pos2,
    pub color: Color32,
}
