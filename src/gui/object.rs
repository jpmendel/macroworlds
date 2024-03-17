use crate::interpreter::state::object::TurtleShape;
use eframe::egui::{vec2, Color32, Pos2, Vec2};

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
    pub size: Vec2,
    pub shape: TurtleShape,
    pub is_visible: bool,
}

impl TurtleView {
    pub fn new(pos: Pos2) -> Self {
        TurtleView {
            pos,
            heading: 270.0, // Translates to facing north
            color: Color32::from_gray(0),
            size: vec2(20.0, 20.0),
            shape: TurtleShape::Triangle,
            is_visible: true,
        }
    }
}

pub struct TextView {
    pub pos: Pos2,
    pub text: String,
    pub font_size: f32,
    pub color: Color32,
    pub is_bold: bool,
    pub is_italic: bool,
    pub is_underlined: bool,
    pub is_visible: bool,
}

impl TextView {
    pub fn new(pos: Pos2) -> Self {
        TextView {
            pos,
            text: String::from("New Text"),
            font_size: 12.0,
            color: Color32::from_gray(0),
            is_bold: false,
            is_italic: false,
            is_underlined: false,
            is_visible: true,
        }
    }
}
