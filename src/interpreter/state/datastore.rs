use crate::interpreter::language::procedure::Procedure;
use crate::interpreter::language::token::Token;
use crate::interpreter::state::object::TurtleShape;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct DataStore {
    scopes: VecDeque<Scope>,
    procedures: HashMap<Box<str>, Procedure>,
    shapes: HashMap<Box<str>, TurtleShape>,
    base_file_directory: String,
    last_error_message: String,
}

impl DataStore {
    pub fn new() -> Self {
        let global_scope = Scope {
            variables: HashMap::new(),
        };
        DataStore {
            scopes: VecDeque::from([global_scope]),
            procedures: HashMap::new(),
            shapes: [
                (Box::from("triangle"), TurtleShape::Triangle),
                (Box::from("circle"), TurtleShape::Circle),
                (Box::from("square"), TurtleShape::Square),
            ]
            .into_iter()
            .collect(),
            base_file_directory: String::new(),
            last_error_message: String::new(),
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

    pub fn reached_max_scope_depth(&self) -> bool {
        // Limit the number of nested scopes to prevent stack overflow.
        self.scopes.len() >= 100
    }

    pub fn get_variable(&self, name: &str) -> Option<&Token> {
        // Go through each scope from most local to most global
        // and search for the variable name in question.
        for scope in &self.scopes {
            if let Some(value) = scope.variables.get(name) {
                return Some(value);
            }
        }
        None
    }

    pub fn set_variable(&mut self, name: &str, value: Token) {
        for scope in &mut self.scopes {
            // If the variable already exists in local scope, allow setting there.
            // Otherwise, all variable sets are in global scope.
            if let Some(..) = scope.variables.get(name) {
                scope.variables.insert(Box::from(name), value);
                return;
            }
        }
        let global_scope = self.scopes.back_mut().unwrap();
        global_scope.variables.insert(Box::from(name), value);
    }

    pub fn init_local(&mut self, name: &str, value: Token) {
        // Initializes a variable in the local scope with a default value
        // only if it does not exist already.
        let local_scope = self.scopes.front_mut().unwrap();
        if !local_scope.variables.contains_key(name) {
            local_scope.variables.insert(Box::from(name), value);
        }
    }

    pub fn remove_variable(&mut self, name: &str) {
        let current_scope = self.scopes.front_mut().unwrap();
        current_scope.variables.remove(name);
    }

    pub fn remove_all_variables_in_scope(&mut self) {
        let current_scope = self.scopes.front_mut().unwrap();
        current_scope.variables.clear();
    }

    pub fn get_procedure(&mut self, name: &str) -> Option<&Procedure> {
        self.procedures.get(name)
    }

    pub fn get_all_procedures(&mut self) -> Vec<&Procedure> {
        self.procedures.iter().map(|(_, value)| value).collect()
    }

    pub fn set_procedure(&mut self, procedure: Procedure) {
        self.procedures.insert(procedure.name.clone(), procedure);
    }

    pub fn get_shape(&self, name: &str) -> Option<&TurtleShape> {
        self.shapes.get(name)
    }

    pub fn set_shape(&mut self, name: &str, shape: TurtleShape) {
        self.shapes.insert(Box::from(name), shape);
    }

    pub fn get_base_directory(&self) -> &String {
        &self.base_file_directory
    }

    pub fn set_base_directory(&mut self, dir: String) {
        if dir.ends_with('/') {
            self.base_file_directory = dir;
        } else {
            self.base_file_directory = format!("{}/", dir);
        }
    }

    pub fn get_last_error_message(&self) -> String {
        self.last_error_message.clone()
    }

    pub fn set_last_error_message(&mut self, message: String) {
        self.last_error_message = message;
    }
}

#[derive(Debug)]
struct Scope {
    variables: HashMap<Box<str>, Token>,
}

impl Scope {
    fn new() -> Self {
        Scope {
            variables: HashMap::new(),
        }
    }
}
