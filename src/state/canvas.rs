use crate::state::state::State;
use crate::state::turtle::{Line, Turtle};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Canvas {
    pub turtles: Vec<Turtle>,
    pub turtle_lookup: HashMap<String, usize>,
    pub current_turtle_index: usize,
    pub lines: Vec<Line>,
}

impl Canvas {
    pub fn new() -> Self {
        let turtle_name = String::from("t1");
        Canvas {
            turtles: vec![Turtle::with(turtle_name.clone())],
            turtle_lookup: [(turtle_name, 0)].into_iter().collect(),
            current_turtle_index: 0,
            lines: vec![],
        }
    }
}

impl State {
    pub fn current_turtle(&mut self) -> &mut Turtle {
        self.canvas
            .turtles
            .get_mut(self.canvas.current_turtle_index)
            .unwrap()
    }

    pub fn set_current_turtle(&mut self, name: &String) -> bool {
        if let Some(index) = self.canvas.turtle_lookup.get(name) {
            self.canvas.current_turtle_index = index.clone();
            true
        } else {
            false
        }
    }

    pub fn create_turtle(&mut self, name: String) -> &Turtle {
        let turtle = Turtle::with(name.clone());
        self.canvas.turtles.push(turtle);
        self.canvas
            .turtle_lookup
            .insert(name, self.canvas.turtles.len() - 1);
        self.canvas.turtles.last().unwrap()
    }

    pub fn remove_turtle(&mut self, name: &String) {
        if let Some(index) = self.canvas.turtle_lookup.get(name) {
            let index = index.clone();
            self.canvas.turtles.remove(index);
            self.canvas.turtle_lookup.remove(name);
            for (index, turtle) in self.canvas.turtles.iter().enumerate() {
                self.canvas.turtle_lookup.insert(turtle.name.clone(), index);
            }
            if self.canvas.current_turtle_index == index {
                self.canvas.current_turtle_index = 0;
            }
        }
    }

    pub fn add_line(&mut self, start: (f32, f32), end: (f32, f32), color: f32) -> &Line {
        let line = Line { start, end, color };
        self.canvas.lines.push(line);
        self.canvas.lines.last().unwrap()
    }
}
