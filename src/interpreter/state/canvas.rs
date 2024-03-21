use crate::interpreter::language::token::Token;
use crate::interpreter::state::object::{Line, Object, Point, Size, Text, Turtle};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub struct CanvasState {
    size: Size,
    pixels: Vec<u8>,
    bg_color: u8,
    objects: HashMap<Box<str>, Object>,
    current_object_name: Box<str>,
}

impl CanvasState {
    pub const DEFAULT_WIDTH: f32 = 600.0;
    pub const DEFAULT_HEIGHT: f32 = 400.0;

    pub fn new() -> Self {
        let pixel_count = (Self::DEFAULT_WIDTH * Self::DEFAULT_HEIGHT) as usize;
        CanvasState {
            size: Size::new(Self::DEFAULT_WIDTH, Self::DEFAULT_HEIGHT),
            pixels: vec![0; pixel_count],
            bg_color: 255,
            objects: HashMap::new(),
            current_object_name: Box::from(""),
        }
    }

    pub fn current_object(&self) -> Result<&Object, Box<dyn Error>> {
        if let Some(object) = self.objects.get(&self.current_object_name) {
            Ok(object)
        } else {
            Err(Box::from(format!(
                "object {} does not exist",
                self.current_object_name
            )))
        }
    }

    pub fn current_object_mut(&mut self) -> Result<&mut Object, Box<dyn Error>> {
        if let Some(object) = self.objects.get_mut(&self.current_object_name) {
            Ok(object)
        } else {
            Err(Box::from(format!(
                "object {} does not exist",
                self.current_object_name
            )))
        }
    }

    pub fn set_current_object(&mut self, name: &str) -> bool {
        if self.objects.get(name).is_some() {
            self.current_object_name = Box::from(name);
            true
        } else {
            false
        }
    }

    pub fn get_object(&self, name: &str) -> Result<&Object, Box<dyn Error>> {
        if let Some(object) = self.objects.get(name) {
            Ok(object)
        } else {
            Err(Box::from(format!("object {} does not exist", name)))
        }
    }

    pub fn create_turtle(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        if self.objects.get(name).is_some() {
            return Err(Box::from(format!("object {} already exists", name)));
        }
        let turtle = Turtle::new(Box::from(name));
        self.objects.insert(Box::from(name), Object::Turtle(turtle));
        if self.objects.len() == 1 {
            self.current_object_name = Box::from(name);
        }
        Ok(())
    }

    pub fn create_text(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        if self.objects.get(name).is_some() {
            return Err(Box::from(format!("object {} already exists", name)));
        }
        let text = Text::new(Box::from(name));
        self.objects.insert(Box::from(name), Object::Text(text));
        if self.objects.len() == 1 {
            self.current_object_name = Box::from(name);
        }
        Ok(())
    }

    pub fn remove_object(&mut self, name: &str) {
        self.objects.remove(name);
        if let Some((name, _)) = self.objects.iter().next() {
            self.current_object_name = name.clone();
        } else {
            self.current_object_name = Box::from("");
        }
    }

    pub fn init_backpack_property(&mut self, name: &str) {
        for (_, obj) in &mut self.objects {
            if let Object::Turtle(turtle) = obj {
                let default_value = Token::Word(String::new());
                turtle.backpack.insert(Box::from(name), default_value);
            }
        }
    }

    pub fn get_size(&self) -> &Size {
        &self.size
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
        let color = self.pixels.get(index).unwrap_or(&self.bg_color);
        if *color == 0 {
            return self.bg_color as f32;
        }
        color.clone() as f32
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
        self.pixels = vec![0; (self.size.w * self.size.h) as usize];
    }
}
