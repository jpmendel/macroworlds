use std::f32::consts::E;

use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::language::command::command::{Command, Params};
use crate::interpreter::language::token::Token;
use crate::interpreter::language::util::{
    are_tokens_equal, ascii_for_key, decode_boolean, decode_list, decode_number, decode_token,
    decode_word, join_to_list_string, key_for_ascii,
};

impl Command {
    pub fn sum() -> Self {
        Command::reserved(
            "sum",
            Params::Variadic(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let mut result = decode_number(com, &args, 0)?;
                for index in 1..args.len() {
                    let num = decode_number(com, &args, index)?;
                    result += num;
                }
                Ok(Token::Number(result))
            },
        )
    }

    pub fn difference() -> Self {
        Command::reserved(
            "difference",
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let num1 = decode_number(com, &args, 0)?;
                let num2 = decode_number(com, &args, 1)?;
                let result = num1 - num2;
                Ok(Token::Number(result))
            },
        )
    }

    pub fn product() -> Self {
        Command::reserved(
            "product",
            Params::Variadic(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let mut result = decode_number(com, &args, 0)?;
                for index in 1..args.len() {
                    let num = decode_number(com, &args, index)?;
                    result *= num;
                }
                Ok(Token::Number(result))
            },
        )
    }

    pub fn quotient() -> Self {
        Command::reserved(
            "quotient",
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let num1 = decode_number(com, &args, 0)?;
                let num2 = decode_number(com, &args, 1)?;
                if num2 == 0.0 {
                    return Err(Box::from("quotient cannot divide by zero"));
                }
                let result = num1 / num2;
                Ok(Token::Number(result))
            },
        )
    }

    pub fn remainder() -> Self {
        Command::reserved(
            "remainder",
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let num1 = decode_number(com, &args, 0)?;
                let num2 = decode_number(com, &args, 1)?;
                let result = num1 % num2;
                Ok(Token::Number(result))
            },
        )
    }

    pub fn power() -> Self {
        Command::reserved(
            "power",
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let num1 = decode_number(com, &args, 0)?;
                let num2 = decode_number(com, &args, 1)?;
                let result = num1.powf(num2);
                Ok(Token::Number(result))
            },
        )
    }

    pub fn sqrt() -> Self {
        Command::reserved(
            "sqrt",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let number = decode_number(com, &args, 0)?;
                if number < 0.0 {
                    return Err(Box::from("cannot sqrt negative number"));
                }
                let result = number.sqrt();
                Ok(Token::Number(result))
            },
        )
    }

    pub fn minus() -> Self {
        Command::reserved(
            "minus",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let number = decode_number(com, &args, 0)?;
                let result = -number;
                Ok(Token::Number(result))
            },
        )
    }

    pub fn abs() -> Self {
        Command::reserved(
            "abs",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let number = decode_number(com, &args, 0)?;
                let result = number.abs();
                Ok(Token::Number(result))
            },
        )
    }

    pub fn int() -> Self {
        Command::reserved(
            "int",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let number = decode_number(com, &args, 0)?;
                Ok(Token::Number(number.floor()))
            },
        )
    }

    pub fn round() -> Self {
        Command::reserved(
            "round",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let number = decode_number(com, &args, 0)?;
                Ok(Token::Number(number.round()))
            },
        )
    }

    pub fn sin() -> Self {
        Command::reserved(
            "sin",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let number = decode_number(com, &args, 0)?;
                Ok(Token::Number(number.sin().to_degrees()))
            },
        )
    }

    pub fn cos() -> Self {
        Command::reserved(
            "cos",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let number = decode_number(com, &args, 0)?;
                Ok(Token::Number(number.cos().to_degrees()))
            },
        )
    }

    pub fn tan() -> Self {
        Command::reserved(
            "tan",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let number = decode_number(com, &args, 0)?;
                Ok(Token::Number(number.tan().to_degrees()))
            },
        )
    }

    pub fn arctan() -> Self {
        Command::reserved(
            "arctan",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let number = decode_number(com, &args, 0)?;
                Ok(Token::Number(number.atan().to_degrees()))
            },
        )
    }

    pub fn exp() -> Self {
        Command::reserved(
            "exp",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let number = decode_number(com, &args, 0)?;
                Ok(Token::Number(E.powf(number)))
            },
        )
    }

    pub fn ln() -> Self {
        Command::reserved(
            "ln",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let number = decode_number(com, &args, 0)?;
                Ok(Token::Number(number.ln()))
            },
        )
    }

    pub fn log() -> Self {
        Command::reserved(
            "log",
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let base = decode_number(com, &args, 0)?;
                let number = decode_number(com, &args, 1)?;
                Ok(Token::Number(number.log(base)))
            },
        )
    }

    pub fn equal() -> Self {
        Command::reserved(
            "equal?",
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let arg1 = decode_token(com, &args, 0)?;
                let arg2 = decode_token(com, &args, 1)?;
                let result = are_tokens_equal(&arg1, &arg2);
                Ok(Token::Boolean(result))
            },
        )
    }

    pub fn greater() -> Self {
        Command::reserved(
            "greater?",
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let arg1 = decode_number(com, &args, 0)?;
                let arg2 = decode_number(com, &args, 1)?;
                let result = arg1 > arg2;
                Ok(Token::Boolean(result))
            },
        )
    }

    pub fn less() -> Self {
        Command::reserved(
            "less?",
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let arg1 = decode_number(com, &args, 0)?;
                let arg2 = decode_number(com, &args, 1)?;
                let result = arg1 < arg2;
                Ok(Token::Boolean(result))
            },
        )
    }

    pub fn or() -> Self {
        Command::reserved(
            "or",
            Params::Variadic(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let mut result = decode_boolean(com, &args, 0)?;
                for index in 1..args.len() {
                    let bool = decode_boolean(com, &args, index)?;
                    result = result || bool;
                }
                Ok(Token::Boolean(result))
            },
        )
    }

    pub fn and() -> Self {
        Command::reserved(
            "and",
            Params::Variadic(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let mut result = decode_boolean(com, &args, 0)?;
                for index in 1..args.len() {
                    let bool = decode_boolean(com, &args, index)?;
                    result = result && bool;
                }
                Ok(Token::Boolean(result))
            },
        )
    }

    pub fn not() -> Self {
        Command::reserved(
            "not",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let boolean = decode_boolean(com, &args, 0)?;
                let result = !boolean;
                Ok(Token::Boolean(result))
            },
        )
    }

    pub fn isnumber() -> Self {
        Command::reserved(
            "number?",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let token = decode_token(com, &args, 0)?;
                let result = match token {
                    Token::Number(..) => true,
                    _ => false,
                };
                Ok(Token::Boolean(result))
            },
        )
    }

    pub fn isword() -> Self {
        Command::reserved(
            "word?",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let token = decode_token(com, &args, 0)?;
                let result = match token {
                    Token::Word(..) => true,
                    _ => false,
                };
                Ok(Token::Boolean(result))
            },
        )
    }

    pub fn islist() -> Self {
        Command::reserved(
            "list?",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let token = decode_token(com, &args, 0)?;
                let result = match token {
                    Token::List(..) => true,
                    _ => false,
                };
                Ok(Token::Boolean(result))
            },
        )
    }

    pub fn word() -> Self {
        Command::reserved(
            "word",
            Params::Variadic(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let mut result = decode_word(com, &args, 0)?;
                for index in 1..args.len() {
                    let word = decode_word(com, &args, index)?;
                    result += &word;
                }
                Ok(Token::Word(result))
            },
        )
    }

    pub fn ascii() -> Self {
        Command::reserved(
            "ascii",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let word = decode_word(com, &args, 0)?;
                let result = ascii_for_key(&word)?;
                Ok(Token::Number(result as f32))
            },
        )
    }

    pub fn char() -> Self {
        Command::reserved(
            "char",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let number = decode_number(com, &args, 0)? as u8;
                let result = key_for_ascii(number)?;
                Ok(Token::Word(result))
            },
        )
    }

    pub fn list() -> Self {
        Command::reserved(
            "list",
            Params::Variadic(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let mut result = decode_token(com, &args, 0)?.to_string();
                for index in 1..args.len() {
                    let word = decode_token(com, &args, index)?.to_string();
                    result += &format!(" {}", word);
                }
                Ok(Token::List(result))
            },
        )
    }

    pub fn count() -> Self {
        Command::reserved(
            "count",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let token = decode_token(com, &args, 0)?;
                match token {
                    Token::List(list) => {
                        let items = int.parse_list(&list, false)?;
                        Ok(Token::Number(items.len() as f32))
                    }
                    Token::Word(word) => Ok(Token::Number(word.len() as f32)),
                    _ => Err(Box::from("count expected a word or list as input")),
                }
            },
        )
    }

    pub fn item() -> Self {
        Command::reserved(
            "item",
            Params::Fixed(2),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let index = decode_number(com, &args, 0)? as usize;
                let token = decode_token(com, &args, 1)?;
                match token {
                    Token::List(list) => {
                        let items = int.parse_list(&list, true)?;
                        if let Some(item) = items.get(index) {
                            Ok(item.clone())
                        } else {
                            let message = format!("item couldn't find index {} in list", index);
                            Err(Box::from(message))
                        }
                    }
                    Token::Word(word) => {
                        if let Some(chr) = word.chars().nth(index) {
                            Ok(Token::Word(chr.to_string()))
                        } else {
                            let message = format!("item couldn't find index {} in word", index);
                            Err(Box::from(message))
                        }
                    }
                    _ => Err(Box::from("item expected a word or list for input 1")),
                }
            },
        )
    }

    pub fn first() -> Self {
        Command::reserved(
            "first",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let list = decode_list(com, &args, 0)?;
                let items = int.parse_list(&list, true)?;
                if let Some(first) = items.first() {
                    Ok(first.clone())
                } else {
                    Err(Box::from("first cannot get from empty list"))
                }
            },
        )
    }

    pub fn last() -> Self {
        Command::reserved(
            "last",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let list = decode_list(com, &args, 0)?;
                let items = int.parse_list(&list, true)?;
                if let Some(last) = items.last() {
                    Ok(last.clone())
                } else {
                    Err(Box::from("last cannot get from empty list"))
                }
            },
        )
    }

    pub fn butfirst() -> Self {
        Command::reserved(
            "butfirst",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let list = decode_list(com, &args, 0)?;
                let items = int.parse_list(&list, false)?;
                if items.is_empty() {
                    return Err(Box::from("butfirst cannot get from empty list"));
                }
                let rest = &items[1..];
                let joined = join_to_list_string(rest.to_vec());
                Ok(Token::List(joined))
            },
        )
    }

    pub fn butlast() -> Self {
        Command::reserved(
            "butlast",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let list = decode_list(com, &args, 0)?;
                let items = int.parse_list(&list, false)?;
                if items.is_empty() {
                    return Err(Box::from("butlast cannot get from empty list"));
                }
                let rest = &items[..items.len() - 1];
                let joined = join_to_list_string(rest.to_vec());
                Ok(Token::List(joined))
            },
        )
    }

    pub fn fput() -> Self {
        Command::reserved(
            "fput",
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let item = decode_token(com, &args, 0)?;
                let list = decode_list(com, &args, 1)?;
                let result = format!("{} {}", item.to_string(), list);
                Ok(Token::List(result))
            },
        )
    }

    pub fn lput() -> Self {
        Command::reserved(
            "lput",
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let item = decode_token(com, &args, 0)?;
                let list = decode_list(com, &args, 1)?;
                let result = format!("{} {}", list, item.to_string());
                Ok(Token::List(result))
            },
        )
    }

    pub fn member() -> Self {
        Command::reserved(
            "member?",
            Params::Fixed(2),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let item = decode_token(com, &args, 0)?;
                let list = decode_list(com, &args, 1)?;
                let list_items = int.parse_list(&list, true)?;
                let result = list_items.contains(&item);
                Ok(Token::Boolean(result))
            },
        )
    }

    pub fn empty() -> Self {
        Command::reserved(
            "empty?",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let token = decode_token(com, &args, 0)?;
                let result = match token {
                    Token::Word(word) => word.is_empty(),
                    Token::List(list) => list.is_empty(),
                    _ => return Err(Box::from("empty? expected a word or list as input")),
                };
                Ok(Token::Boolean(result))
            },
        )
    }
}
