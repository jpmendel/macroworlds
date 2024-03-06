use crate::view::turtle::{Line, Turtle};
use eframe::egui::*;
use std::collections::HashMap;

pub struct Canvas {
    pub pos: Pos2,
    pub size: Vec2,
    pub turtles: Vec<Turtle>,
    pub turtle_lookup: HashMap<String, usize>,
    pub current_turtle_index: usize,
    pub lines: Vec<Line>,
}

impl Canvas {
    pub fn new(pos: Pos2, size: Vec2) -> Self {
        let turtle = Turtle {
            pos: pos2(pos.x + size.x / 2.0, pos.y + size.y / 2.0),
            heading: 0.0,
            color: Color32::from_gray(0),
            is_visible: true,
            is_drawing: true,
        };
        Canvas {
            pos,
            size,
            turtles: vec![turtle],
            turtle_lookup: [(String::from("t1"), 0)].into_iter().collect(),
            current_turtle_index: 0,
            lines: vec![],
        }
    }

    pub fn current_turtle(&mut self) -> &mut Turtle {
        self.turtles.get_mut(self.current_turtle_index).unwrap()
    }

    pub fn add_turtle(&mut self, name: String, turtle: Turtle) {
        self.turtles.push(turtle);
        self.turtle_lookup.insert(name, self.turtles.len() - 1);
    }

    pub fn to_canvas_coordinates(&self, x: f32, y: f32) -> (f32, f32) {
        (
            x + (self.pos.x + self.size.x / 2.0),
            -y + (self.pos.y + self.size.y / 2.0),
        )
    }
}
