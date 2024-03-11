use crate::language::token::Token;
use crate::state::object::{CanvasObject, Line, Point, Size, Text, Turtle};
use crate::state::state::State;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug)]
pub struct Canvas {
    size: Size,
    pixels: Vec<u8>,
    bg_color: u8,
    objects: HashMap<String, CanvasObject>,
    current_object_name: String,
    turtle_backpack: HashSet<String>,
}

impl Canvas {
    pub fn new() -> Self {
        let name = String::from("t1");
        let turtle = Turtle::with(name.clone());
        let pixel_count = (State::DEFAULT_CANVAS_WIDTH * State::DEFAULT_CANVAS_HEIGHT) as usize;
        Canvas {
            size: Size::from(
                State::DEFAULT_CANVAS_WIDTH.clone(),
                State::DEFAULT_CANVAS_HEIGHT.clone(),
            ),
            pixels: vec![255; pixel_count],
            bg_color: 255,
            objects: [(name.clone(), CanvasObject::Turtle(turtle))]
                .into_iter()
                .collect(),
            current_object_name: name,
            turtle_backpack: HashSet::new(),
        }
    }

    pub fn current_object(&mut self) -> Result<&mut CanvasObject, Box<dyn Error>> {
        if let Some(obj) = self.objects.get_mut(&self.current_object_name) {
            Ok(obj)
        } else {
            Err(Box::from("current object does not exist"))
        }
    }

    pub fn current_turtle(&mut self) -> Result<&mut Turtle, Box<dyn Error>> {
        if let Some(CanvasObject::Turtle(turtle)) = self.objects.get_mut(&self.current_object_name)
        {
            Ok(turtle)
        } else {
            Err(Box::from("current object is not a turtle"))
        }
    }

    pub fn current_text(&mut self) -> Result<&mut Text, Box<dyn Error>> {
        if let Some(CanvasObject::Text(text)) = self.objects.get_mut(&self.current_object_name) {
            Ok(text)
        } else {
            Err(Box::from("current object is not a text"))
        }
    }

    pub fn set_current_object(&mut self, name: String) -> bool {
        if self.objects.get(&name).is_some() {
            self.current_object_name = name;
            true
        } else {
            false
        }
    }

    pub fn get_turtle(&mut self, name: &String) -> Result<&mut Turtle, Box<dyn Error>> {
        if let Some(CanvasObject::Turtle(turtle)) = self.objects.get_mut(name) {
            Ok(turtle)
        } else {
            Err(Box::from(format!("turtle {} does not exist", name)))
        }
    }

    pub fn create_turtle(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        if self.objects.get(&name).is_some() {
            return Err(Box::from(format!("object {} already exists", name)));
        }
        let turtle = Turtle::with(name.clone());
        self.objects
            .insert(name.clone(), CanvasObject::Turtle(turtle));
        if self.objects.len() == 1 {
            self.current_object_name = name;
        }
        Ok(())
    }

    pub fn create_text(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        if self.objects.get(&name).is_some() {
            return Err(Box::from(format!("object {} already exists", name)));
        }
        let text = Text::with(name.clone());
        self.objects.insert(name.clone(), CanvasObject::Text(text));
        if self.objects.len() == 1 {
            self.current_object_name = name;
        }
        Ok(())
    }

    pub fn remove_object(&mut self, name: &String) {
        self.objects.remove(name);
        if let Some((name, _)) = self.objects.iter().next() {
            self.current_object_name = name.clone();
        } else {
            self.current_object_name = String::new();
        }
    }

    pub fn init_backpack_property(&mut self, name: String) {
        self.turtle_backpack.insert(name.clone());
        for (_, obj) in &mut self.objects {
            if let CanvasObject::Turtle(turtle) = obj {
                turtle
                    .backpack
                    .insert(name.clone(), Token::Word(String::new()));
            }
        }
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.size.w = width;
        self.size.h = height;
        self.pixels = vec![self.bg_color; (width * height) as usize];
    }

    pub fn get_bg_color(&self) -> u8 {
        self.bg_color
    }

    pub fn set_bg_color(&mut self, color: u8) {
        self.bg_color = color;
    }

    pub fn color_at_point(&self, point: &Point) -> f32 {
        let x = (point.x + self.size.w / 2.0) as i32;
        let y = (point.y + self.size.h / 2.0) as i32;
        let index = (y * (self.size.w as i32) + x) as usize;
        self.pixels.get(index).unwrap_or(&self.bg_color).clone() as f32
    }

    pub fn add_line(&mut self, line: Line) {
        let color = line.color as u8;
        let stroke = line.stroke_width as i32;
        let start_x = (line.start.x + self.size.w / 2.0) as i32;
        let end_x = (line.end.x + self.size.w / 2.0) as i32;
        let start_y = (line.start.y + self.size.h / 2.0) as i32;
        let end_y = (line.end.y + self.size.h / 2.0) as i32;
        let (x1, x2) = if start_x < end_x {
            (start_x, end_x)
        } else {
            (end_x + 1, start_x + 1)
        };
        let (y1, y2) = if start_y < end_y {
            (start_y, end_y)
        } else {
            (end_y + 1, start_y + 1)
        };
        let x_diff = x2 - x1;
        let y_diff = y2 - y1;
        if x_diff == 0 {
            for step in 0..y_diff {
                let y = y1 + step;
                let span = stroke - 1;
                for i in -span..=span {
                    let index = (y * (self.size.w as i32) + x1 + i) as usize;
                    if let Some(pixel) = self.pixels.get_mut(index) {
                        *pixel = color;
                    }
                }
            }
        } else if y_diff == 0 {
            for step in 0..x_diff {
                let x = x1 + step;
                let span = stroke - 1;
                for i in -span..=span {
                    let index = ((y1 + i) * (self.size.w as i32) + x) as usize;
                    if let Some(pixel) = self.pixels.get_mut(index) {
                        *pixel = color;
                    }
                }
            }
        } else {
            let slope = y_diff / x_diff;
            for step in 0..x_diff {
                let x = x1 + step;
                let y = y1 + slope * step;
                let span = stroke - 1;
                for i in -span..=span {
                    for j in -span..=span {
                        let index = ((y + j) * (self.size.w as i32) + x + i) as usize;
                        if let Some(pixel) = self.pixels.get_mut(index) {
                            *pixel = color;
                        }
                    }
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.pixels = vec![self.bg_color; (self.size.w * self.size.h) as usize];
    }
}
