use crate::language::command::Command;
use std::error::Error;

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

    pub fn to_number(&self) -> Result<Token, Box<dyn Error>> {
        match self {
            Self::Word(word) => {
                if let Ok(number) = word.parse::<f32>() {
                    Ok(Token::Number(number))
                } else {
                    Err(Box::from(format!("cannot convert {} to number", word)))
                }
            }
            Self::Number(..) => Ok(self.clone()),
            _ => Err(Box::from("cannot convert token to number")),
        }
    }
}
