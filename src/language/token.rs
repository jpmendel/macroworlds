use crate::language::command::Command;

#[derive(Debug, Clone)]
pub enum Token {
    Command(Command, Vec<Token>),
    Word(String),
    Number(f32),
    Boolean(bool),
    List(String),
    Variable(String),
    Procedure(String, Vec<String>, String),
    Undefined(String),
    Void,
}
