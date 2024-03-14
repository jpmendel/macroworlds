use crate::interpreter::event::{UiContext, UiEvent, UiEventHandler};
use crate::view::canvas::CanvasView;
use crate::view::object::{ObjectView, TextView, TurtleView};
use eframe::egui::*;
use eframe::epaint::PathShape;
use std::sync::{Arc, Mutex};

impl UiEventHandler for CanvasView {
    fn handle_ui_event(&mut self, ctx: Arc<Mutex<dyn UiContext>>, event: UiEvent) {
        match event {
            UiEvent::Done => {
                let ctx = ctx.lock().unwrap();
                ctx.update_ui();
            }
            UiEvent::Wait(..) => {
                let ctx = ctx.lock().unwrap();
                ctx.update_ui();
            }
            UiEvent::ConsolePrint(text) => {
                self.print_to_console(text);
            }
            UiEvent::Announce(text) => {
                self.announce_text = text;
                self.is_window_open = true;
            }
            UiEvent::NewTurtle(name) => {
                if let None = self.objects.get(&name) {
                    let turtle = TurtleView::new(pos2(0.0, 0.0));
                    self.objects.insert(name, ObjectView::Turtle(turtle));
                } else {
                    self.print_to_console(format!("object named {} already exists", name));
                }
            }
            UiEvent::NewText(name) => {
                if let None = self.objects.get(&name) {
                    let text = TextView::new(pos2(0.0, 0.0));
                    self.objects.insert(name, ObjectView::Text(text));
                } else {
                    self.print_to_console(format!("object named {} already exists", name));
                }
            }
            UiEvent::RemoveObject(name) => {
                self.objects.remove(&name);
            }
            UiEvent::ObjectPos(name, point) => {
                let pos = pos2(point.x, point.y);
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
            UiEvent::TurtleSize(name, size) => {
                if let Some(ObjectView::Turtle(turtle)) = self.objects.get_mut(&name) {
                    turtle.size = size;
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
            UiEvent::TextPrint(name, text_string) => {
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
                let start = self.to_canvas_coordinates(pos2(line.start.x, line.start.y));
                let end = self.to_canvas_coordinates(pos2(line.end.x, line.end.y));
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
                let path = PathShape::line(vec![start, end], Stroke::new(line.stroke_width, color));
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

impl UiContext for Context {
    fn update_ui(&self) {
        self.request_repaint();
    }
}
