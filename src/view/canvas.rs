use crate::interpreter::state::object::TurtleShape;
use crate::view::object::{ObjectView, TurtleView};
use eframe::egui::*;
use eframe::epaint::{CircleShape, Hsva, PathShape, RectShape};
use std::collections::HashMap;

pub struct CanvasView {
    pub pos: Pos2,
    pub size: Vec2,
    pub objects: HashMap<Box<str>, ObjectView>,
    pub bg_color: Color32,
    pub current_turtle_paths: HashMap<Box<str>, PathShape>,
    pub drawn_paths: Vec<PathShape>,
    pub console_text: String,
    pub announce_text: String,
    pub is_window_open: bool,
}

impl CanvasView {
    pub fn new(size: Vec2) -> Self {
        let turtle = TurtleView::new(pos2(0.0, 0.0));
        CanvasView {
            pos: pos2(0.0, 0.0),
            size,
            objects: [(Box::from("t1"), ObjectView::Turtle(turtle))]
                .into_iter()
                .collect(),
            bg_color: Color32::from_gray(255),
            current_turtle_paths: HashMap::new(),
            drawn_paths: vec![],
            console_text: String::new(),
            announce_text: String::new(),
            is_window_open: false,
        }
    }

    pub fn to_canvas_coordinates(&self, pos: Pos2) -> Pos2 {
        // Translate from "(0, 0) center, north positive" system to the rect on the page.
        pos2(
            pos.x + (self.pos.x + self.size.x / 2.0),
            -pos.y + (self.pos.y + self.size.y / 2.0),
        )
    }

    pub fn to_canvas_angle(&self, angle: f32) -> f32 {
        // Translate from "clockwise, 0 == north" to "counterclockwise, 0 == east" system.
        (-angle - 90.0) % 360.0
    }

    pub fn to_canvas_color(&self, hue: f32) -> Color32 {
        if hue == 1.0 {
            Color32::from_gray(0)
        } else if hue == 0.0 || hue == 255.0 {
            Color32::from_gray(255)
        } else {
            Color32::from(Hsva::new((hue % 256.0) / 255.0, 1.0, 1.0, 1.0))
        }
    }

    pub fn print_to_console(&mut self, text: String) {
        if self.console_text.is_empty() {
            self.console_text = text;
        } else {
            self.console_text.insert_str(0, &format!("{}\n", text));
        }
    }

    pub fn shape_for_turtle(&self, turtle: &TurtleView) -> Shape {
        let pos = self.to_canvas_coordinates(turtle.pos);
        let size = turtle.size;
        match turtle.shape {
            TurtleShape::Triangle => Shape::Path(PathShape::convex_polygon(
                vec![
                    pos2(
                        pos.x - size * turtle.heading.to_radians().cos(),
                        pos.y + size * turtle.heading.to_radians().sin(),
                    ),
                    pos2(
                        pos.x - size * ((turtle.heading + 120.0) % 360.0).to_radians().cos(),
                        pos.y + size * ((turtle.heading + 120.0) % 360.0).to_radians().sin(),
                    ),
                    pos2(
                        pos.x - size * ((turtle.heading + 240.0) % 360.0).to_radians().cos(),
                        pos.y + size * ((turtle.heading + 240.0) % 360.0).to_radians().sin(),
                    ),
                ],
                turtle.color,
                Stroke::new(1.0, turtle.color),
            )),
            TurtleShape::Circle => Shape::Circle(CircleShape::filled(pos, size, turtle.color)),
            TurtleShape::Square => Shape::Rect(RectShape::filled(
                Rect::from_center_size(pos, vec2(size * 2.0, size * 2.0)),
                Rounding::default(),
                turtle.color,
            )),
        }
    }
}
