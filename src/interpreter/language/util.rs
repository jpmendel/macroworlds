use crate::interpreter::language::token::Token;
use std::error::Error;

pub fn decode_number(com: &str, args: &Vec<Token>, index: usize) -> Result<f32, Box<dyn Error>> {
    if let Some(Token::Number(num)) = args.get(index) {
        Ok(num.clone())
    } else {
        Err(Box::from(format!(
            "{} expected number for input {}",
            com, index
        )))
    }
}

pub fn decode_word(com: &str, args: &Vec<Token>, index: usize) -> Result<String, Box<dyn Error>> {
    if let Some(Token::Word(word)) = args.get(index) {
        Ok(word.clone())
    } else {
        Err(Box::from(format!(
            "{} expected word for input {}",
            com, index
        )))
    }
}

pub fn decode_boolean(com: &str, args: &Vec<Token>, index: usize) -> Result<bool, Box<dyn Error>> {
    if let Some(Token::Boolean(boolean)) = args.get(index) {
        Ok(boolean.clone())
    } else {
        Err(Box::from(format!(
            "{} expected boolean for input {}",
            com, index
        )))
    }
}

pub fn decode_list(com: &str, args: &Vec<Token>, index: usize) -> Result<String, Box<dyn Error>> {
    if let Some(Token::List(list)) = args.get(index) {
        Ok(list.clone())
    } else {
        Err(Box::from(format!(
            "{} expected list for input {}",
            com, index
        )))
    }
}

pub fn decode_proc(
    com: &str,
    args: &Vec<Token>,
    index: usize,
) -> Result<(String, Vec<String>, String), Box<dyn Error>> {
    if let Some(Token::Procedure(name, params, code)) = args.get(index) {
        Ok((name.clone(), params.clone(), code.clone()))
    } else {
        Err(Box::from(format!(
            "{} expected code block for input {}",
            com, index
        )))
    }
}

pub fn decode_token(com: &str, args: &Vec<Token>, index: usize) -> Result<Token, Box<dyn Error>> {
    if let Some(token) = args.get(index) {
        Ok(token.clone())
    } else {
        Err(Box::from(format!("{} expected an input at {}", com, index)))
    }
}

pub fn are_tokens_equal(arg1: &Token, arg2: &Token) -> bool {
    match (arg1, arg2) {
        (Token::Number(num1), Token::Number(num2)) => num1 == num2,
        (Token::Word(str1), Token::Word(str2)) => str1 == str2,
        (Token::Boolean(bool1), Token::Boolean(bool2)) => bool1 == bool2,
        _ => false,
    }
}

pub fn join_to_list_string(tokens: Vec<Token>) -> String {
    let mut list = String::new();
    for token in tokens {
        match token {
            Token::Word(word) => list += &word,
            Token::Number(number) => list += &number.to_string(),
            Token::List(other) => list += &format!("[{}]", other),
            Token::Boolean(bool) => list += &bool.to_string(),
            _ => continue,
        }
        list.push(' ');
    }
    list.trim().to_string()
}
