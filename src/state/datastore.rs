use crate::language::command::Procedure;
use crate::language::token::Token;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct DataStore {
    scopes: VecDeque<Scope>,
    procedures: HashMap<Box<str>, Procedure>,
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
                scope.variables.insert(name.into(), value);
                return;
            }
        }
        let global_scope = self.scopes.back_mut().unwrap();
        global_scope.variables.insert(name.into(), value);
    }

    pub fn set_local(&mut self, name: &str, value: Token) {
        let local_scope = self.scopes.front_mut().unwrap();
        local_scope.variables.insert(name.into(), value);
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
