use crate::interpreter::state::object::TurtleShape;
use eframe::egui::{Color32, Pos2};

pub enum ObjectView {
    Turtle(TurtleView),
    Text(TextView),
}

impl ObjectView {
    pub fn set_pos(&mut self, pos: Pos2) {
        match self {
            ObjectView::Turtle(turtle) => turtle.pos = pos,
            ObjectView::Text(text) => text.pos = pos,
        }
    }

    pub fn set_color(&mut self, color: Color32) {
        match self {
            ObjectView::Turtle(turtle) => turtle.color = color,
            ObjectView::Text(text) => text.color = color,
        }
    }

    pub fn set_visible(&mut self, is_visible: bool) {
        match self {
            ObjectView::Turtle(turtle) => turtle.is_visible = is_visible,
            ObjectView::Text(text) => text.is_visible = is_visible,
        }
    }
}

pub struct TurtleView {
    pub pos: Pos2,
    pub heading: f32,
    pub color: Color32,
    pub shape: TurtleShape,
    pub is_visible: bool,
}

impl TurtleView {
    pub fn new(pos: Pos2) -> Self {
        TurtleView {
            pos,
            heading: 270.0,
            color: Color32::from_gray(0),
            shape: TurtleShape::Triangle,
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
    pub fn new(pos: Pos2) -> Self {
        TextView {
            pos,
            color: Color32::from_gray(0),
            is_visible: true,
            text: String::from("New Text"),
            font_size: 12.0,
        }
    }
}
