use crate::interpreter::event::UiEvent;
use crate::state::object::TurtleShape;
use crate::view::object::{ObjectView, TextView, TurtleView};
use eframe::egui::*;
use eframe::epaint::{CircleShape, Hsva, PathShape, RectShape};
use std::collections::HashMap;
use std::f32::consts::PI;

pub struct CanvasView {
    pub pos: Pos2,
    pub size: Vec2,
    pub objects: HashMap<String, ObjectView>,
    pub bg_color: Color32,
    pub current_turtle_paths: HashMap<String, PathShape>,
    pub drawn_paths: Vec<PathShape>,
    pub console_text: String,
}

impl CanvasView {
    pub fn with(size: Vec2) -> Self {
        let turtle = TurtleView::with(pos2(0.0, 0.0));
        CanvasView {
            pos: pos2(0.0, 0.0),
            size,
            objects: [(String::from("t1"), ObjectView::Turtle(turtle))]
                .into_iter()
                .collect(),
            bg_color: Color32::from_gray(255),
            current_turtle_paths: HashMap::new(),
            drawn_paths: vec![],
            console_text: String::new(),
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
        if hue == 0.0 {
            Color32::from_gray(0)
        } else if hue == 255.0 {
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
        match turtle.shape {
            TurtleShape::Triangle => Shape::Path(PathShape::convex_polygon(
                vec![
                    pos2(
                        pos.x - 8.0 * (turtle.heading * PI / 180.0).cos(),
                        pos.y + 8.0 * (turtle.heading * PI / 180.0).sin(),
                    ),
                    pos2(
                        pos.x - 8.0 * (((turtle.heading + 120.0) % 360.0) * PI / 180.0).cos(),
                        pos.y + 8.0 * (((turtle.heading + 120.0) % 360.0) * PI / 180.0).sin(),
                    ),
                    pos2(
                        pos.x - 8.0 * (((turtle.heading + 240.0) % 360.0) * PI / 180.0).cos(),
                        pos.y + 8.0 * (((turtle.heading + 240.0) % 360.0) * PI / 180.0).sin(),
                    ),
                ],
                turtle.color,
                Stroke::new(1.0, turtle.color),
            )),
            TurtleShape::Circle => Shape::Circle(CircleShape::filled(pos, 8.0, turtle.color)),
            TurtleShape::Square => Shape::Rect(RectShape::filled(
                Rect::from_center_size(pos, vec2(16.0, 16.0)),
                Rounding::default(),
                turtle.color,
            )),
        }
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
                self.print_to_console(text);
            }
            UiEvent::NewTurtle(name) => {
                if let None = self.objects.get(&name) {
                    let turtle = TurtleView::with(pos2(0.0, 0.0));
                    self.objects.insert(name, ObjectView::Turtle(turtle));
                } else {
                    self.print_to_console(format!("object named {} already exists", name));
                }
            }
            UiEvent::NewText(name) => {
                if let None = self.objects.get(&name) {
                    let text = TextView::with(pos2(0.0, 0.0));
                    self.objects.insert(name, ObjectView::Text(text));
                } else {
                    self.print_to_console(format!("object named {} already exists", name));
                }
            }
            UiEvent::RemoveObject(name) => {
                self.objects.remove(&name);
            }
            UiEvent::ObjectPos(name, (x, y)) => {
                let pos = pos2(x, y);
                if let Some(obj) = self.objects.get_mut(&name) {
                    obj.set_pos(pos);
                } else {
                    self.print_to_console(format!("object named {} does not exist", name));
                }
            }
            UiEvent::ObjectColor(name, hue) => {
                let color = self.to_canvas_color(hue);
                if let Some(obj) = self.objects.get_mut(&name) {
                    obj.set_color(color)
                } else {
                    self.print_to_console(format!("object named {} does not exist", name));
                }
            }
            UiEvent::ObjectVisible(name, is_visible) => {
                if let Some(obj) = self.objects.get_mut(&name) {
                    obj.set_visible(is_visible);
                } else {
                    self.print_to_console(format!("object named {} does not exist", name));
                }
            }
            UiEvent::TurtleHeading(name, angle) => {
                let heading = self.to_canvas_angle(angle);
                if let Some(ObjectView::Turtle(turtle)) = self.objects.get_mut(&name) {
                    turtle.heading = heading;
                } else {
                    self.print_to_console(format!("turtle named {} does not exist", name));
                }
            }
            UiEvent::TurtleShape(name, shape) => {
                if let Some(ObjectView::Turtle(turtle)) = self.objects.get_mut(&name) {
                    turtle.shape = shape;
                } else {
                    self.print_to_console(format!("turtle named {} does not exist", name));
                }
            }
            UiEvent::TextAddText(name, text_string) => {
                if let Some(ObjectView::Text(text)) = self.objects.get_mut(&name) {
                    text.text += &text_string;
                } else {
                    self.print_to_console(format!("text named {} does not exist", name));
                }
            }
            UiEvent::TextClear(name) => {
                if let Some(ObjectView::Text(text)) = self.objects.get_mut(&name) {
                    text.text = String::new();
                } else {
                    self.print_to_console(format!("text named {} does not exist", name));
                }
            }
            UiEvent::TextSize(name, font_size) => {
                if let Some(ObjectView::Text(text)) = self.objects.get_mut(&name) {
                    text.font_size = font_size;
                } else {
                    self.print_to_console(format!("text named {} does not exist", name));
                }
            }
            UiEvent::CanvasSize(width, height) => {
                self.size.x = width;
                self.size.y = height;
            }
            UiEvent::BgColor(hue) => {
                let color = self.to_canvas_color(hue);
                self.bg_color = color;
            }
            UiEvent::AddLine(name, line) => {
                let start = self.to_canvas_coordinates(pos2(line.start.0, line.start.1));
                let end = self.to_canvas_coordinates(pos2(line.end.0, line.end.1));
                let color = self.to_canvas_color(line.color);
                if let Some(path) = self.current_turtle_paths.get_mut(&name) {
                    if path.stroke.color == color {
                        if let Some(point) = path.points.last() {
                            if *point == start {
                                path.points.push(end);
                            }
                        }
                    }
                    if let Some(path) = self.current_turtle_paths.remove(&name) {
                        self.drawn_paths.push(path);
                    }
                }
                let path = PathShape::line(vec![start, end], Stroke::new(3.0, color));
                self.current_turtle_paths.insert(name, path);
            }
            UiEvent::Clean => {
                self.current_turtle_paths.clear();
                self.drawn_paths.clear();
            }
            UiEvent::ClearConsole => {
                self.console_text = String::new();
            }
        };
    }
}
