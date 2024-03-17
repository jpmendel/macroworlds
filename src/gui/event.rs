use crate::gui::canvas::{CanvasView, PathConfig};
use crate::gui::object::{ObjectView, TextView, TurtleView};
use crate::interpreter::event::{UiContext, UiEvent, UiEventHandler};
use eframe::egui::*;
use std::any::Any;
use std::error::Error;
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
            UiEvent::ObjectSize(name, size) => {
                if let Some(ObjectView::Turtle(turtle)) = self.objects.get_mut(&name) {
                    turtle.size = vec2(size.w, size.h);
                } else {
                    self.print_to_console(format!("object named {} does not exist", name));
                }
            }
            UiEvent::TurtleHeading(name, angle) => {
                let heading = self.to_canvas_angle(angle);
                if let Some(ObjectView::Turtle(turtle)) = self.objects.get_mut(&name) {
                    turtle.heading = heading;
                } else {
                    self.print_to_console(format!("object named {} does not exist", name));
                }
            }
            UiEvent::TurtleShape(name, shape) => {
                if let Some(ObjectView::Turtle(turtle)) = self.objects.get_mut(&name) {
                    turtle.shape = shape;
                } else {
                    self.print_to_console(format!("object named {} does not exist", name));
                }
            }
            UiEvent::TextPrint(name, text_string) => {
                if let Some(ObjectView::Text(text)) = self.objects.get_mut(&name) {
                    text.text += &text_string;
                } else {
                    self.print_to_console(format!("object named {} does not exist", name));
                }
            }
            UiEvent::TextClear(name) => {
                if let Some(ObjectView::Text(text)) = self.objects.get_mut(&name) {
                    text.text = String::new();
                } else {
                    self.print_to_console(format!("object named {} does not exist", name));
                }
            }
            UiEvent::TextSize(name, font_size) => {
                if let Some(ObjectView::Text(text)) = self.objects.get_mut(&name) {
                    text.font_size = font_size;
                } else {
                    self.print_to_console(format!("object named {} does not exist", name));
                }
            }
            UiEvent::TextStyle(name, is_bold, is_italic, is_underlined) => {
                if let Some(ObjectView::Text(text)) = self.objects.get_mut(&name) {
                    text.is_bold = is_bold;
                    text.is_italic = is_italic;
                    text.is_underlined = is_underlined;
                } else {
                    self.print_to_console(format!("object named {} does not exist", name));
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
                let start = pos2(line.start.x, line.start.y);
                let end = pos2(line.end.x, line.end.y);
                let color = self.to_canvas_color(line.color);
                if let Some(path) = self.current_turtle_paths.get_mut(&name) {
                    if path.color == color {
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
                let path = PathConfig {
                    points: vec![start, end],
                    color,
                    stroke: line.stroke_width,
                };
                self.current_turtle_paths.insert(name, path);
            }
            UiEvent::AddShape(name, path) => {
                let ctx = ctx.lock().unwrap();
                let result = match ctx.load_image(name.clone(), path) {
                    Ok(result) => result,
                    Err(err) => {
                        self.print_to_console(format!("failed to load image: {}", err));
                        return;
                    }
                };
                let Ok(handle) = result.downcast::<TextureHandle>() else {
                    self.print_to_console(String::from("failed to load image"));
                    return;
                };
                self.image_textures.insert(name, *handle);
            }
            UiEvent::SetPicture(path) => {
                let path_ptr = path.clone().into_boxed_str();
                if let Some(picture) = self.image_textures.get(&path_ptr) {
                    self.bg_picture = Some(picture.clone());
                    return;
                }
                let ctx = ctx.lock().unwrap();
                let result = match ctx.load_image(path_ptr.clone(), path) {
                    Ok(result) => result,
                    Err(err) => {
                        self.print_to_console(format!("failed to load image: {}", err));
                        return;
                    }
                };
                let Ok(handle) = result.downcast::<TextureHandle>() else {
                    self.print_to_console(String::from("failed to load image"));
                    return;
                };
                self.image_textures.insert(path_ptr, *handle.clone());
                self.bg_picture = Some(*handle);
            }
            UiEvent::Clean => {
                self.current_turtle_paths.clear();
                self.drawn_paths.clear();
                self.bg_picture = None;
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

    fn load_image(&self, name: Box<str>, path: String) -> Result<Box<dyn Any>, Box<dyn Error>> {
        let Ok(reader) = image::io::Reader::open(path.clone()) else {
            return Err(Box::from(format!("could not open path: {}", path)));
        };
        let Ok(image) = reader.decode() else {
            return Err(Box::from("invalid image format"));
        };
        let size = [image.width() as usize, image.height() as usize];
        let buffer = image.to_rgba8();
        let pixels = buffer.as_flat_samples();
        let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
        let texture = self.load_texture(name, color_image, TextureOptions::LINEAR);
        Ok(Box::from(texture))
    }
}
