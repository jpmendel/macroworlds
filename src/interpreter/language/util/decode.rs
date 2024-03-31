use crate::interpreter::language::structure::Procedure;
use crate::interpreter::language::token::Token;
use std::error::Error;

pub fn number(com: &str, args: &Vec<Token>, index: usize) -> Result<f32, Box<dyn Error>> {
    if let Some(Token::Number(num)) = args.get(index) {
        Ok(num.clone())
    } else {
        Err(Box::from(format!(
            "{} expected a number for input {}",
            com, index
        )))
    }
}

pub fn word(com: &str, args: &Vec<Token>, index: usize) -> Result<String, Box<dyn Error>> {
    if let Some(Token::Word(word)) = args.get(index) {
        Ok(word.clone())
    } else {
        Err(Box::from(format!(
            "{} expected a word for input {}",
            com, index
        )))
    }
}

pub fn boolean(com: &str, args: &Vec<Token>, index: usize) -> Result<bool, Box<dyn Error>> {
    if let Some(Token::Boolean(boolean)) = args.get(index) {
        Ok(boolean.clone())
    } else {
        Err(Box::from(format!(
            "{} expected a boolean for input {}",
            com, index
        )))
    }
}

pub fn list(com: &str, args: &Vec<Token>, index: usize) -> Result<String, Box<dyn Error>> {
    if let Some(Token::List(list)) = args.get(index) {
        Ok(list.clone())
    } else {
        Err(Box::from(format!(
            "{} expected a list for input {}",
            com, index
        )))
    }
}

pub fn procedure(com: &str, args: &Vec<Token>, index: usize) -> Result<Procedure, Box<dyn Error>> {
    if let Some(Token::Procedure(name, params, code)) = args.get(index) {
        Ok(Procedure {
            name: Box::from(name.as_str()),
            params: params.clone(),
            code: code.clone(),
        })
    } else {
        Err(Box::from(format!(
            "{} expected a code block for input {}",
            com, index
        )))
    }
}

pub fn token(com: &str, args: &Vec<Token>, index: usize) -> Result<Token, Box<dyn Error>> {
    if let Some(token) = args.get(index) {
        Ok(token.clone())
    } else {
        Err(Box::from(format!("{} expected an input at {}", com, index)))
    }
}
