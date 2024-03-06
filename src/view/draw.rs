use crate::language::event::UiEvent;
use crate::view::canvas::Canvas;
use crate::view::turtle::Line;
use eframe::egui::{pos2, Color32, Context};
use eframe::epaint::Hsva;

use super::turtle::Turtle;

impl Canvas {
    pub fn draw(&mut self, ctx: &Context, event: UiEvent) {
        match event {
            UiEvent::Done => {
                ctx.request_repaint();
            }
            UiEvent::Wait(..) => {
                ctx.request_repaint();
            }
            UiEvent::Fd(amount) => {
                let turtle = self.current_turtle();
                let original_pos = turtle.pos.clone();
                let x = amount * turtle.heading.to_radians().cos();
                let y = amount * turtle.heading.to_radians().sin();
                let new_pos = pos2(original_pos.x + x, original_pos.y + y);
                turtle.pos = new_pos.clone();
                let color = turtle.color.clone();
                self.lines.push(Line {
                    start: original_pos,
                    end: new_pos,
                    color,
                });
            }
            UiEvent::Bk(amount) => {
                let turtle = self.current_turtle();
                let original_pos = turtle.pos.clone();
                let x = -amount * turtle.heading.to_radians().cos();
                let y = -amount * turtle.heading.to_radians().sin();
                let new_pos = pos2(original_pos.x + x, original_pos.y + y);
                turtle.pos = new_pos.clone();
                let color = turtle.color.clone();
                self.lines.push(Line {
                    start: original_pos,
                    end: new_pos,
                    color,
                });
            }
            UiEvent::Lt(amount) => {
                let turtle = self.current_turtle();
                turtle.heading -= amount;
            }
            UiEvent::Rt(amount) => {
                let turtle = self.current_turtle();
                turtle.heading += amount;
            }
            UiEvent::Setpos(x, y) => {
                let (new_x, new_y) = self.to_canvas_coordinates(x, y);
                let turtle = self.current_turtle();
                turtle.pos.x = new_x;
                turtle.pos.y = new_y;
            }
            UiEvent::Seth(heading) => {
                let turtle = self.current_turtle();
                turtle.heading = heading;
            }
            UiEvent::Setc(color) => {
                let turtle = self.current_turtle();
                turtle.color = Color32::from(Hsva::new(color / 255.0, 1.0, 1.0, 1.0));
            }
            UiEvent::Pd => {
                let turtle = self.current_turtle();
                turtle.is_drawing = true;
            }
            UiEvent::Pu => {
                let turtle = self.current_turtle();
                turtle.is_drawing = false;
            }
            UiEvent::St => {
                let turtle = self.current_turtle();
                turtle.is_visible = true;
            }
            UiEvent::Ht => {
                let turtle = self.current_turtle();
                turtle.is_visible = false;
            }
            UiEvent::Clean => {
                self.lines.clear();
            }
            UiEvent::Addr(name) => {
                if let Some(index) = self.turtle_lookup.get(&name) {
                    self.current_turtle_index = index.clone();
                } else {
                    println!("error: no turtle named {}", name);
                }
            }
            UiEvent::Newturtle(name) => {
                if let None = self.turtle_lookup.get(&name) {
                    let turtle = Turtle {
                        pos: pos2(
                            self.pos.x + self.size.x / 2.0,
                            self.pos.y + self.size.y / 2.0,
                        ),
                        heading: 0.0,
                        color: Color32::from_gray(0),
                        is_visible: true,
                        is_drawing: true,
                    };
                    self.add_turtle(name, turtle);
                } else {
                    println!("error: turtle named {} already exists", name);
                }
            }
        };
    }
}
