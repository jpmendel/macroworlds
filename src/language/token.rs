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

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            Self::Command(command, _) => command.name.clone(),
            Self::Word(string) => string.clone(),
            Self::Number(number) => number.to_string(),
            Self::Boolean(boolean) => boolean.to_string(),
            Self::List(list) => format!("[{}]", list),
            Self::Undefined(undef) => undef.clone(),
            _ => String::new(),
        }
    }
}
