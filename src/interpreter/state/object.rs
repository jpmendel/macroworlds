use crate::interpreter::language::token::Token;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum Object {
    Turtle(Turtle),
    Text(Text),
}

impl Object {
    pub fn name(&self) -> &str {
        match self {
            Self::Turtle(turtle) => &turtle.name,
            Self::Text(text) => &text.name,
        }
    }

    pub fn pos(&self) -> &Point {
        match self {
            Self::Turtle(turtle) => &turtle.pos,
            Self::Text(text) => &text.pos,
        }
    }

    pub fn set_pos(&mut self, pos: Point) {
        match self {
            Self::Turtle(turtle) => turtle.pos = pos,
            Self::Text(text) => text.pos = pos,
        }
    }

    pub fn color(&self) -> f32 {
        match self {
            Self::Turtle(turtle) => turtle.color,
            Self::Text(text) => text.color,
        }
    }

    pub fn set_color(&mut self, color: f32) {
        match self {
            Self::Turtle(turtle) => turtle.color = color,
            Self::Text(text) => text.color = color,
        }
    }

    pub fn is_visible(&self) -> bool {
        match self {
            Self::Turtle(turtle) => turtle.is_visible,
            Self::Text(text) => text.is_visible,
        }
    }

    pub fn set_visible(&mut self, is_visible: bool) {
        match self {
            Self::Turtle(turtle) => turtle.is_visible = is_visible,
            Self::Text(text) => text.is_visible = is_visible,
        }
    }

    pub fn is_locked(&self) -> bool {
        match self {
            Self::Turtle(turtle) => turtle.is_locked,
            Self::Text(text) => text.is_locked,
        }
    }

    pub fn set_locked(&mut self, is_locked: bool) {
        match self {
            Self::Turtle(turtle) => turtle.is_locked = is_locked,
            Self::Text(text) => text.is_locked = is_locked,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Turtle {
    pub name: Box<str>,
    pub pos: Point,
    pub heading: f32,
    pub color: f32,
    pub size: Size,
    pub pen_size: f32,
    pub shape: TurtleShape,
    pub is_visible: bool,
    pub is_drawing: bool,
    pub is_locked: bool,
    pub backpack: HashMap<Box<str>, Token>,
}

impl Turtle {
    pub fn new(name: Box<str>) -> Self {
        Turtle {
            name,
            pos: Point::zero(),
            heading: 0.0,
            color: 1.0, // Black
            size: Size::equal(20.0),
            pen_size: 1.0,
            shape: TurtleShape::Triangle,
            is_visible: true,
            is_drawing: true,
            is_locked: false,
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
    Image(Box<str>, String),
}

impl TurtleShape {
    pub fn to_string(&self) -> String {
        match self {
            Self::Triangle => String::from("triangle"),
            Self::Circle => String::from("circle"),
            Self::Square => String::from("square"),
            Self::Image(name, _) => format!("image:{}", name),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Text {
    pub name: Box<str>,
    pub pos: Point,
    pub text: String,
    pub font_size: f32,
    pub color: f32,
    pub style: HashSet<TextStyle>,
    pub is_visible: bool,
    pub is_locked: bool,
}

impl Text {
    pub fn new(name: Box<str>) -> Self {
        Text {
            name,
            pos: Point::zero(),
            text: String::from("New Text"),
            font_size: 12.0,
            color: 1.0, // Black
            style: HashSet::new(),
            is_visible: true,
            is_locked: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextStyle {
    Bold,
    Italic,
    Underline,
}

impl TextStyle {
    pub fn from(string: String) -> Option<Self> {
        match string.as_str() {
            "bold" => Some(Self::Bold),
            "italic" => Some(Self::Italic),
            "underline" => Some(Self::Underline),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

#[derive(Debug, Clone)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

impl Size {
    pub fn new(w: f32, h: f32) -> Self {
        Size { w, h }
    }

    pub fn equal(side: f32) -> Self {
        Self::new(side, side)
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
    pub fn new(start: Point, end: Point, color: f32, stroke_width: f32) -> Self {
        Line {
            start,
            end,
            color,
            stroke_width,
        }
    }
}
