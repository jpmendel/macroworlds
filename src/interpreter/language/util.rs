use crate::interpreter::language::procedure::Procedure;
use crate::interpreter::language::token::Token;
use std::error::Error;
use std::fs::{self, DirEntry};

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
) -> Result<Procedure, Box<dyn Error>> {
    if let Some(Token::Procedure(name, params, code)) = args.get(index) {
        Ok(Procedure {
            name: Box::from(name.as_str()),
            params: params.clone(),
            code: code.clone(),
        })
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
            Token::Word(word) => {
                if word.contains(' ') {
                    list += &format!("|{}|", word);
                } else {
                    list += &word
                }
            }
            Token::Number(number) => list += &number.to_string(),
            Token::List(other) => list += &format!("[{}]", other),
            Token::Boolean(bool) => list += &bool.to_string(),
            _ => continue,
        }
        list.push(' ');
    }
    list.trim().to_string()
}

pub fn ascii_for_key(key: &str) -> Result<u8, Box<dyn Error>> {
    let ascii = match key {
        "space" => 32,
        "enter" => 10,
        "left" => 37,
        "up" => 38,
        "right" => 39,
        "down" => 40,
        chr if chr.len() == 1 => chr.chars().next().unwrap().to_ascii_lowercase() as u8,
        _ => return Err(Box::from("key is not an ascii character")),
    };
    Ok(ascii)
}

pub fn key_for_ascii(ascii: u8) -> Result<String, Box<dyn Error>> {
    let key = match ascii {
        32 => String::from("space"),
        10 => String::from("enter"),
        37 => String::from("left"),
        38 => String::from("up"),
        39 => String::from("right"),
        40 => String::from("down"),
        num if num.is_ascii() => String::from(num as char),
        _ => return Err(Box::from("number does not represent an ascii key")),
    };
    Ok(key)
}

pub fn query_files(
    base_path: &String,
    query: fn(&DirEntry) -> bool,
) -> Result<String, Box<dyn Error>> {
    let mut files: Vec<String> = vec![];
    let entries = fs::read_dir(base_path)?;
    for entry in entries {
        let Ok(entry) = entry else { continue };
        if (query)(&entry) {
            let Ok(file_name) = entry.file_name().into_string() else {
                continue;
            };
            files.push(file_name);
        }
    }
    Ok(files.join(" "))
}
