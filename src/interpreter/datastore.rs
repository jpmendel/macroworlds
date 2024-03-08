use crate::language::command::Procedure;
use crate::language::token::Token;
use crate::language::turtle::{Line, Turtle};
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Datastore {
    scopes: VecDeque<Scope>,
    procedures: HashMap<String, Procedure>,
    canvas: Canvas,
    input: InputState,
}

impl Datastore {
    pub fn new() -> Self {
        let global_scope = Scope {
            variables: HashMap::new(),
        };
        Datastore {
            scopes: VecDeque::from([global_scope]),
            procedures: HashMap::new(),
            canvas: Canvas::new(),
            input: InputState::new(),
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push_front(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        // Prevent popping of global scope.
        if self.scopes.len() > 1 {
            self.scopes.pop_front();
        }
    }

    pub fn reset_scope(&mut self) {
        // Pop all except the global scope.
        while self.scopes.len() > 1 {
            self.scopes.pop_front();
        }
    }

    pub fn get_variable(&self, name: &String) -> Option<&Token> {
        // Go through each scope from most local to most global
        // and search for the variable name in question.
        for scope in &self.scopes {
            if let Some(value) = scope.variables.get(name) {
                return Some(value);
            }
        }
        None
    }

    pub fn set_variable(&mut self, name: String, value: Token) {
        for scope in &mut self.scopes {
            // If the variable already exists in local scope, allow setting there.
            // Otherwise, all variable sets are in global scope.
            if let Some(..) = scope.variables.get(&name) {
                scope.variables.insert(name, value);
                return;
            }
        }
        let global_scope = self.scopes.back_mut().unwrap();
        global_scope.variables.insert(name, value);
    }

    pub fn set_local(&mut self, name: String) {
        let local_scope = self.scopes.front_mut().unwrap();
        local_scope.variables.insert(name, Token::Void);
    }

    pub fn remove_variable(&mut self, name: &str) {
        let current_scope = self.scopes.front_mut().unwrap();
        current_scope.variables.remove(name);
    }

    pub fn get_procedure(&mut self, name: &str) -> Option<&Procedure> {
        self.procedures.get(name)
    }

    pub fn set_procedure(&mut self, procedure: Procedure) {
        self.procedures.insert(procedure.name.clone(), procedure);
    }

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

    pub fn get_turtle(&mut self, name: &String) -> Option<&Turtle> {
        let index = self.canvas.turtle_lookup.get(name)?;
        self.canvas.turtles.get(*index)
    }

    pub fn create_turtle(&mut self, name: String) -> &Turtle {
        let turtle = Turtle::with(name.clone());
        self.canvas.turtles.push(turtle);
        self.canvas
            .turtle_lookup
            .insert(name, self.canvas.turtles.len() - 1);
        self.canvas.turtles.last().unwrap()
    }

    pub fn add_line(&mut self, start: (f32, f32), end: (f32, f32), color: f32) -> &Line {
        let line = Line { start, end, color };
        self.canvas.lines.push(line);
        self.canvas.lines.last().unwrap()
    }

    pub fn get_one_key(&mut self) -> Option<String> {
        self.input.key_buffer.pop_front()
    }

    pub fn add_key_to_buffer(&mut self, key: String) {
        self.input.key_buffer.push_back(key);
    }
}

#[derive(Debug)]
struct Scope {
    variables: HashMap<String, Token>,
}

impl Scope {
    fn new() -> Self {
        Scope {
            variables: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct Canvas {
    turtles: Vec<Turtle>,
    turtle_lookup: HashMap<String, usize>,
    current_turtle_index: usize,
    lines: Vec<Line>,
}

impl Canvas {
    fn new() -> Self {
        let turtle_name = String::from("t1");
        Canvas {
            turtles: vec![Turtle::with(turtle_name.clone())],
            turtle_lookup: [(turtle_name, 0)].into_iter().collect(),
            current_turtle_index: 0,
            lines: vec![],
        }
    }
}

#[derive(Debug)]
struct InputState {
    key_buffer: VecDeque<String>,
}

impl InputState {
    fn new() -> Self {
        InputState {
            key_buffer: VecDeque::new(),
        }
    }
}
