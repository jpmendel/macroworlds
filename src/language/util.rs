use crate::language::token::Token;
use std::error::Error;

pub fn decode_number(arg: Option<&Token>) -> Result<f32, Box<dyn Error>> {
    let number: f32;
    if let Some(Token::Number(num)) = arg {
        number = num.clone();
    } else {
        return Err(Box::from("expected number as input"));
    }
    Ok(number)
}

pub fn decode_string(arg: Option<&Token>) -> Result<String, Box<dyn Error>> {
    let string: String;
    if let Some(Token::String(s)) = arg {
        string = s.clone().replacen('"', "", 1);
    } else {
        return Err(Box::from("expected string as input"));
    }
    Ok(string)
}

pub fn decode_boolean(arg: Option<&Token>) -> Result<bool, Box<dyn Error>> {
    let boolean: bool;
    if let Some(Token::Boolean(b)) = arg {
        boolean = b.clone();
    } else {
        return Err(Box::from("expected boolean as input"));
    }
    Ok(boolean)
}

pub fn decode_list(arg: Option<&Token>) -> Result<String, Box<dyn Error>> {
    let array: String;
    if let Some(Token::List(arr)) = arg {
        array = arr.clone().replace('[', "").replace(']', "");
    } else {
        return Err(Box::from("expected list as input"));
    }
    Ok(array)
}

pub fn decode_proc(arg: Option<&Token>) -> Result<(String, Vec<String>, String), Box<dyn Error>> {
    if let Some(Token::Procedure(name, params, code)) = arg {
        Ok((name.clone(), params.clone(), code.clone()))
    } else {
        Err(Box::from("expected block as input"))
    }
}

pub fn decode_token(arg: Option<&Token>) -> Result<&Token, Box<dyn Error>> {
    if let Some(token) = arg {
        Ok(token)
    } else {
        Err(Box::from("expected an input"))
    }
}

pub fn are_tokens_equal(arg1: &Token, arg2: &Token) -> bool {
    match (arg1, arg2) {
        (Token::Number(num1), Token::Number(num2)) => num1 == num2,
        (Token::String(str1), Token::String(str2)) => str1 == str2,
        (Token::Boolean(bool1), Token::Boolean(bool2)) => bool1 == bool2,
        _ => false,
    }
}
