use crate::language::command::Command;

#[derive(Debug, Clone)]
pub enum Token {
    Command(Command, Vec<Token>),
    Variable(String),
    Number(f32),
    String(String),
    Boolean(bool),
    List(String),
    Procedure(String, Vec<String>, String),
    Undefined(String),
    Void,
}
