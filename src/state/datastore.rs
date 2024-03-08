use crate::language::command::Procedure;
use crate::language::token::Token;
use crate::state::state::State;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct DataStore {
    pub scopes: VecDeque<Scope>,
    pub procedures: HashMap<String, Procedure>,
}

#[derive(Debug)]
pub struct Scope {
    variables: HashMap<String, Token>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            variables: HashMap::new(),
        }
    }
}

impl DataStore {
    pub fn new() -> Self {
        let global_scope = Scope {
            variables: HashMap::new(),
        };
        DataStore {
            scopes: VecDeque::from([global_scope]),
            procedures: HashMap::new(),
        }
    }
}

impl State {
    pub fn push_scope(&mut self) {
        self.data.scopes.push_front(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        // Prevent popping of global scope.
        if self.data.scopes.len() > 1 {
            self.data.scopes.pop_front();
        }
    }

    pub fn reset_scope(&mut self) {
        // Pop all except the global scope.
        while self.data.scopes.len() > 1 {
            self.data.scopes.pop_front();
        }
    }

    pub fn get_variable(&self, name: &String) -> Option<&Token> {
        // Go through each scope from most local to most global
        // and search for the variable name in question.
        for scope in &self.data.scopes {
            if let Some(value) = scope.variables.get(name) {
                return Some(value);
            }
        }
        None
    }

    pub fn set_variable(&mut self, name: String, value: Token) {
        for scope in &mut self.data.scopes {
            // If the variable already exists in local scope, allow setting there.
            // Otherwise, all variable sets are in global scope.
            if let Some(..) = scope.variables.get(&name) {
                scope.variables.insert(name, value);
                return;
            }
        }
        let global_scope = self.data.scopes.back_mut().unwrap();
        global_scope.variables.insert(name, value);
    }

    pub fn set_local(&mut self, name: String) {
        let local_scope = self.data.scopes.front_mut().unwrap();
        local_scope.variables.insert(name, Token::Void);
    }

    pub fn remove_variable(&mut self, name: &str) {
        let current_scope = self.data.scopes.front_mut().unwrap();
        current_scope.variables.remove(name);
    }

    pub fn get_procedure(&mut self, name: &str) -> Option<&Procedure> {
        self.data.procedures.get(name)
    }

    pub fn set_procedure(&mut self, procedure: Procedure) {
        self.data
            .procedures
            .insert(procedure.name.clone(), procedure);
    }
}
