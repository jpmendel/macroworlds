use crate::language::command::Procedure;
use crate::language::token::Token;
use std::collections::HashMap;

pub struct Datastore {
    scopes: Vec<Scope>,
    pub procedures: HashMap<String, Procedure>,
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

impl Datastore {
    pub fn new() -> Self {
        let global_scope = Scope {
            variables: HashMap::new(),
        };
        Datastore {
            scopes: vec![global_scope],
            procedures: HashMap::new(),
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.insert(0, Scope::new());
    }

    pub fn pop_scope(&mut self) {
        // Prevent popping of global scope.
        if self.scopes.len() > 1 {
            self.scopes.remove(0);
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
        let global_scope = self.scopes.last_mut().unwrap();
        global_scope.variables.insert(name, value);
    }

    pub fn set_local(&mut self, name: String) {
        let local_scope = self.scopes.first_mut().unwrap();
        local_scope.variables.insert(name, Token::Void);
    }

    pub fn remove_variable(&mut self, name: &str) {
        let current_scope = self.scopes.first_mut().unwrap();
        current_scope.variables.remove(name);
    }

    pub fn get_procedure(&self, name: &str) -> Option<&Procedure> {
        self.procedures.get(name)
    }

    pub fn set_procedure(&mut self, procedure: Procedure) {
        self.procedures.insert(procedure.name.clone(), procedure);
    }
}
