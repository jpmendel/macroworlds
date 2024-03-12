use crate::language::token::Token;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum CanvasObject {
    Turtle(Turtle),
    Text(Text),
}

impl CanvasObject {
    pub fn name(&self) -> &String {
        match self {
            CanvasObject::Turtle(turtle) => &turtle.name,
            CanvasObject::Text(text) => &text.name,
        }
    }

    pub fn pos(&self) -> &Point {
        match self {
            CanvasObject::Turtle(turtle) => &turtle.pos,
            CanvasObject::Text(text) => &text.pos,
        }
    }

    pub fn set_pos(&mut self, pos: Point) {
        match self {
            CanvasObject::Turtle(turtle) => turtle.pos = pos,
            CanvasObject::Text(text) => text.pos = pos,
        }
    }

    pub fn color(&self) -> &f32 {
        match self {
            CanvasObject::Turtle(turtle) => &turtle.color,
            CanvasObject::Text(text) => &text.color,
        }
    }

    pub fn set_color(&mut self, color: f32) {
        match self {
            CanvasObject::Turtle(turtle) => turtle.color = color,
            CanvasObject::Text(text) => text.color = color,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Turtle {
    pub name: String,
    pub pos: Point,
    pub heading: f32,
    pub color: f32,
    pub pen_size: f32,
    pub shape: TurtleShape,
    pub is_visible: bool,
    pub is_drawing: bool,
    pub backpack: HashMap<String, Token>,
}

impl Turtle {
    pub fn with(name: String) -> Self {
        Turtle {
            name,
            pos: Point::zero(),
            heading: 0.0,
            color: 1.0, // Black
            pen_size: 1.0,
            shape: TurtleShape::Triangle,
            is_visible: true,
            is_drawing: true,
            backpack: HashMap::new(),
        }
    }

    pub fn true_heading(&self) -> f32 {
        // Translate heading from a "clockwise, 0 == north" to a "counterclockwise, 0 == east" system.
        (-self.heading + 90.0).to_radians()
    }
}

#[derive(Debug, Clone)]
pub enum TurtleShape {
    Triangle,
    Circle,
    Square,
}

impl TurtleShape {
    pub fn to_string(&self) -> String {
        match self {
            Self::Triangle => String::from("triangle"),
            Self::Circle => String::from("circle"),
            Self::Square => String::from("square"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Text {
    pub name: String,
    pub pos: Point,
    pub color: f32,
    pub is_visible: bool,
    pub text: String,
    pub font_size: f32,
}

impl Text {
    pub fn with(name: String) -> Self {
        Text {
            name,
            pos: Point::zero(),
            color: 1.0, // Black
            is_visible: true,
            text: String::from("New Text"),
            font_size: 12.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn with(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    pub fn zero() -> Self {
        Point { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug, Clone)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

impl Size {
    pub fn from(w: f32, h: f32) -> Self {
        Size { w, h }
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub color: f32,
    pub stroke_width: f32,
}

impl Line {
    pub fn from(start: Point, end: Point, color: f32, stroke_width: f32) -> Self {
        Line {
            start,
            end,
            color,
            stroke_width,
        }
    }
}
