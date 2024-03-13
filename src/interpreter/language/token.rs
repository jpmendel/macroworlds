use crate::interpreter::language::command::command::Command;

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
            Self::Command(command, _) => command.name.to_string(),
            Self::Word(string) => string.clone(),
            Self::Number(number) => number.to_string(),
            Self::Boolean(boolean) => boolean.to_string(),
            Self::List(list) => format!("[{}]", list),
            Self::Undefined(undef) => undef.clone(),
            _ => String::new(),
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Command(com1, _), Self::Command(com2, _)) => com1.name == com2.name,
            (Self::Word(word1), Self::Word(word2)) => word1 == word2,
            (Self::Number(num1), Self::Number(num2)) => num1 == num2,
            (Self::Boolean(bool1), Self::Boolean(bool2)) => bool1 == bool2,
            (Self::List(list1), Self::List(list2)) => list1 == list2,
            (Self::Variable(var1), Self::Variable(var2)) => var1 == var2,
            (Self::Procedure(proc1, _, _), Self::Procedure(proc2, _, _)) => proc1 == proc2,
            (Self::Undefined(undef1), Self::Undefined(undef2)) => undef1 == undef2,
            _ => false,
        }
    }
}

impl Eq for Token {}
