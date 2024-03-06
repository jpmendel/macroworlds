use crate::interpreter::interpreter::Interpreter;
use crate::language::token::Token;
use std::error::Error;

pub type CommandAction = fn(
    data: &mut Interpreter,
    command: &Command,
    args: Vec<Token>,
) -> Result<Token, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub params: Vec<String>,
    pub action: CommandAction,
}

#[derive(Debug, Clone)]
pub struct Procedure {
    pub name: String,
    pub params: Vec<String>,
    pub code: String,
}
