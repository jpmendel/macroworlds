use crate::interpreter::interpreter::Interpreter;
use crate::language::token::Token;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub is_reserved: bool,
    pub params: Params,
    pub action: CommandAction,
}

impl Command {
    pub fn reserved(name: String, params: Params, action: CommandAction) -> Self {
        Command {
            name,
            is_reserved: true,
            params,
            action,
        }
    }

    pub fn user_defined(name: String, params: Params, action: CommandAction) -> Self {
        Command {
            name,
            is_reserved: false,
            params,
            action,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Params {
    Fixed(usize),
    Variadic(usize),
    None,
}

pub type CommandAction =
    fn(data: &mut Interpreter, command: &String, args: Vec<Token>) -> Result<Token, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct Procedure {
    pub name: String,
    pub params: Vec<String>,
    pub code: String,
}
