use eframe::egui::{Color32, Pos2};

pub enum CanvasView {
    Turtle(TurtleView),
    Text(TextView),
}

impl CanvasView {
    pub fn set_pos(&mut self, pos: Pos2) {
        match self {
            CanvasView::Turtle(turtle) => turtle.pos = pos,
            CanvasView::Text(text) => text.pos = pos,
        }
    }

    pub fn set_color(&mut self, color: Color32) {
        match self {
            CanvasView::Turtle(turtle) => turtle.color = color,
            CanvasView::Text(text) => text.color = color,
        }
    }

    pub fn set_visible(&mut self, is_visible: bool) {
        match self {
            CanvasView::Turtle(turtle) => turtle.is_visible = is_visible,
            CanvasView::Text(text) => text.is_visible = is_visible,
        }
    }
}

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
            heading: 90.0,
            color: Color32::from_gray(0),
            is_visible: true,
        }
    }
}

pub struct TextView {
    pub pos: Pos2,
    pub color: Color32,
    pub is_visible: bool,
    pub text: String,
    pub font_size: f32,
}

impl TextView {
    pub fn with(pos: Pos2) -> Self {
        TextView {
            pos,
            color: Color32::from_gray(0),
            is_visible: true,
            text: String::from("New Text"),
            font_size: 12.0,
        }
    }
}

pub struct LineView {
    pub start: Pos2,
    pub end: Pos2,
    pub color: Color32,
}
