use crate::interpreter::event::UiEvent;
use crate::view::object::{CanvasView, LineView, TextView, TurtleView};
use eframe::egui::*;
use eframe::epaint::Hsva;
use std::collections::HashMap;

pub struct Canvas {
    pub pos: Pos2,
    pub size: Vec2,
    pub objects: HashMap<String, CanvasView>,
    pub lines: Vec<LineView>,
    pub console_text: String,
}

impl Canvas {
    pub fn new(pos: Pos2, size: Vec2) -> Self {
        let turtle = TurtleView::with(pos2(pos.x + size.x / 2.0, pos.y + size.y / 2.0));
        Canvas {
            pos,
            size,
            objects: [(String::from("t1"), CanvasView::Turtle(turtle))]
                .into_iter()
                .collect(),
            lines: vec![],
            console_text: String::new(),
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
                self.console_text = text;
            }
            UiEvent::NewTurtle(name) => {
                if let None = self.objects.get(&name) {
                    let turtle = TurtleView::with(self.to_canvas_coordinates(0.0, 0.0));
                    self.objects.insert(name, CanvasView::Turtle(turtle));
                } else {
                    println!("error: turtle named {} already exists", name);
                }
            }
            UiEvent::NewText(name) => {
                if let None = self.objects.get(&name) {
                    let text = TextView::with(self.to_canvas_coordinates(0.0, 0.0));
                    self.objects.insert(name, CanvasView::Text(text));
                } else {
                    println!("error: turtle named {} already exists", name);
                }
            }
            UiEvent::RemoveObject(name) => {
                self.objects.remove(&name);
            }
            UiEvent::ObjectPos(name, (x, y)) => {
                let pos = self.to_canvas_coordinates(x, y);
                if let Some(obj) = self.objects.get_mut(&name) {
                    obj.set_pos(pos);
                } else {
                    println!("error: turtle named {} does not exist", name);
                }
            }
            UiEvent::ObjectColor(name, hue) => {
                if let Some(obj) = self.objects.get_mut(&name) {
                    if hue == 0.0 {
                        obj.set_color(Color32::from_gray(0));
                    } else {
                        obj.set_color(Color32::from(Hsva::new(hue / 255.0, 1.0, 1.0, 1.0)));
                    }
                } else {
                    println!("error: turtle named {} does not exist", name);
                }
            }
            UiEvent::ObjectVisible(name, is_visible) => {
                if let Some(obj) = self.objects.get_mut(&name) {
                    obj.set_visible(is_visible);
                } else {
                    println!("error: turtle named {} does not exist", name);
                }
            }
            UiEvent::TurtleHeading(name, angle) => {
                let heading = self.to_canvas_angle(angle);
                if let Some(CanvasView::Turtle(turtle)) = self.objects.get_mut(&name) {
                    turtle.heading = heading;
                } else {
                    println!("error: turtle named {} does not exist", name);
                }
            }
            UiEvent::TextText(name, text_string) => {
                if let Some(CanvasView::Text(text)) = self.objects.get_mut(&name) {
                    text.text = text_string;
                } else {
                    println!("error: text named {} does not exist", name);
                }
            }
            UiEvent::TextSize(name, font_size) => {
                if let Some(CanvasView::Text(text)) = self.objects.get_mut(&name) {
                    text.font_size = font_size;
                } else {
                    println!("error: text named {} does not exist", name);
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
