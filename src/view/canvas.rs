use crate::interpreter::event::UiEvent;
use crate::view::turtle::{LineView, TurtleView};
use eframe::egui::*;
use eframe::epaint::Hsva;
use std::collections::HashMap;

pub struct Canvas {
    pub pos: Pos2,
    pub size: Vec2,
    pub turtles: HashMap<String, TurtleView>,
    pub lines: Vec<LineView>,
    pub text: String,
}

impl Canvas {
    pub fn new(pos: Pos2, size: Vec2) -> Self {
        let turtle = TurtleView::with(pos2(pos.x + size.x / 2.0, pos.y + size.y / 2.0));
        Canvas {
            pos,
            size,
            turtles: [(String::from("t1"), turtle)].into_iter().collect(),
            lines: vec![],
            text: String::new(),
        }
    }

    pub fn to_canvas_coordinates(&self, x: f32, y: f32) -> Pos2 {
        // Translate from "(0, 0) center, north positive" system to the rect on the page.
        pos2(
            x + (self.pos.x + self.size.x / 2.0),
            -y + (self.pos.y + self.size.y / 2.0),
        )
    }

    pub fn to_canvas_angle(&self, angle: f32) -> f32 {
        // Translate from "clockwise, 0 == north" to "counterclockwise, 0 == east" system.
        (-angle - 90.0) % 360.0
    }

    pub fn handle_ui_event(&mut self, ctx: &Context, event: UiEvent) {
        match event {
            UiEvent::Done => {
                ctx.request_repaint();
            }
            UiEvent::Wait(..) => {
                ctx.request_repaint();
            }
            UiEvent::Print(text) => {
                self.text = text;
            }
            UiEvent::NewTurtle(name) => {
                if let None = self.turtles.get(&name) {
                    let turtle = TurtleView::with(self.to_canvas_coordinates(0.0, 0.0));
                    self.turtles.insert(name, turtle);
                } else {
                    println!("error: turtle named {} already exists", name);
                }
            }
            UiEvent::RemoveTurtle(name) => {
                self.turtles.remove(&name);
            }
            UiEvent::TurtlePos(name, (x, y)) => {
                let pos = self.to_canvas_coordinates(x, y);
                if let Some(turtle) = self.turtles.get_mut(&name) {
                    turtle.pos.x = pos.x;
                    turtle.pos.y = pos.y;
                } else {
                    println!("error: turtle named {} does not exist", name);
                }
            }
            UiEvent::TurtleHeading(name, angle) => {
                let heading = self.to_canvas_angle(angle);
                if let Some(turtle) = self.turtles.get_mut(&name) {
                    turtle.heading = heading;
                } else {
                    println!("error: turtle named {} does not exist", name);
                }
            }
            UiEvent::TurtleColor(name, hue) => {
                if let Some(turtle) = self.turtles.get_mut(&name) {
                    if hue == 0.0 {
                        turtle.color = Color32::from_gray(0);
                    } else {
                        turtle.color = Color32::from(Hsva::new(hue / 255.0, 1.0, 1.0, 1.0));
                    }
                } else {
                    println!("error: turtle named {} does not exist", name);
                }
            }
            UiEvent::TurtleVisible(name, is_visible) => {
                if let Some(turtle) = self.turtles.get_mut(&name) {
                    turtle.is_visible = is_visible;
                } else {
                    println!("error: turtle named {} does not exist", name);
                }
            }
            UiEvent::AddLine(line) => {
                let color = if line.color == 0.0 {
                    Color32::from_gray(0)
                } else {
                    Color32::from(Hsva::new(line.color / 255.0, 1.0, 1.0, 1.0))
                };
                self.lines.push(LineView {
                    start: self.to_canvas_coordinates(line.start.0, line.start.1),
                    end: self.to_canvas_coordinates(line.end.0, line.end.1),
                    color,
                })
            }
            UiEvent::Clean => {
                self.lines.clear();
            }
        };
    }
}
