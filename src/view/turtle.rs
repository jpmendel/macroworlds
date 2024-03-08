use eframe::egui::{Color32, Pos2};

pub struct TurtleView {
    pub pos: Pos2,
    pub heading: f32,
    pub color: Color32,
    pub is_visible: bool,
}

impl TurtleView {
    pub fn with(pos: Pos2) -> Self {
        TurtleView {
            pos,
            heading: 0.0,
            color: Color32::from_gray(0),
            is_visible: true,
        }
    }
}

pub struct LineView {
    pub start: Pos2,
    pub end: Pos2,
    pub color: Color32,
}
