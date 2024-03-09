use crate::language::token::Token;
use crate::state::object::{CanvasObject, Line, Text, Turtle};
use crate::state::state::State;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug)]
pub struct Canvas {
    pub objects: HashMap<String, CanvasObject>,
    pub current_object_name: String,
    pub turtle_backpack: HashSet<String>,
    pub lines: Vec<Line>,
}

impl Canvas {
    pub fn new() -> Self {
        let name = String::from("t1");
        let turtle = Turtle::with(name.clone());
        Canvas {
            objects: [(name.clone(), CanvasObject::Turtle(turtle))]
                .into_iter()
                .collect(),
            current_object_name: name,
            turtle_backpack: HashSet::new(),
            lines: vec![],
        }
    }
}

impl State {
    pub fn current_object(&mut self) -> Result<&mut CanvasObject, Box<dyn Error>> {
        if let Some(obj) = self
            .canvas
            .objects
            .get_mut(&self.canvas.current_object_name)
        {
            Ok(obj)
        } else {
            Err(Box::from("current object does not exist"))
        }
    }

    pub fn current_turtle(&mut self) -> Result<&mut Turtle, Box<dyn Error>> {
        if let Some(CanvasObject::Turtle(turtle)) = self
            .canvas
            .objects
            .get_mut(&self.canvas.current_object_name)
        {
            Ok(turtle)
        } else {
            Err(Box::from("current object is not a turtle"))
        }
    }

    pub fn current_text(&mut self) -> Result<&mut Text, Box<dyn Error>> {
        if let Some(CanvasObject::Text(text)) = self
            .canvas
            .objects
            .get_mut(&self.canvas.current_object_name)
        {
            Ok(text)
        } else {
            Err(Box::from("current object is not a text"))
        }
    }

    pub fn set_current_object(&mut self, name: String) -> bool {
        if self.canvas.objects.get(&name).is_some() {
            self.canvas.current_object_name = name;
            true
        } else {
            false
        }
    }

    pub fn create_turtle(&mut self, name: String) {
        let turtle = Turtle::with(name.clone());
        self.canvas
            .objects
            .insert(name, CanvasObject::Turtle(turtle));
    }

    pub fn create_text(&mut self, name: String) {
        let text = Text::with(name.clone());
        self.canvas.objects.insert(name, CanvasObject::Text(text));
    }

    pub fn remove_object(&mut self, name: &String) {
        self.canvas.objects.remove(name);
        if let Some((name, _)) = self.canvas.objects.iter().next() {
            self.canvas.current_object_name = name.clone();
        } else {
            self.canvas.current_object_name = String::new();
        }
    }

    pub fn init_backpack_property(&mut self, name: String) {
        self.canvas.turtle_backpack.insert(name.clone());
        for (_, obj) in &mut self.canvas.objects {
            if let CanvasObject::Turtle(turtle) = obj {
                turtle
                    .backpack
                    .insert(name.clone(), Token::Word(String::new()));
            }
        }
    }

    pub fn add_line(&mut self, start: (f32, f32), end: (f32, f32), color: f32) -> &Line {
        let line = Line { start, end, color };
        self.canvas.lines.push(line);
        self.canvas.lines.last().unwrap()
    }
}
