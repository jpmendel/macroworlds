use crate::interpreter::interpreter::Interpreter;
use crate::language::token::Token;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub params: Params,
    pub action: CommandAction,
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
