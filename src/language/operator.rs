use crate::interpreter::interpreter::Interpreter;
use crate::language::command::{Command, Params};
use crate::language::token::Token;
use crate::language::util::{
    are_tokens_equal, decode_boolean, decode_list, decode_number, decode_token, decode_word,
    join_to_list_string,
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
                    return Err(Box::from("cannot divide by zero"));
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
                match word.as_str() {
                    "space" => return Ok(Token::Number(32.0)),
                    "enter" => return Ok(Token::Number(10.0)),
                    "left" => return Ok(Token::Number(37.0)),
                    "up" => return Ok(Token::Number(38.0)),
                    "right" => return Ok(Token::Number(39.0)),
                    "down" => return Ok(Token::Number(40.0)),
                    _ => (),
                };
                if word.len() != 1 {
                    return Err(Box::from("input must be a character"));
                }
                let result = word.chars().next().unwrap().to_ascii_lowercase() as u8;
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
                if !number.is_ascii() {
                    return Err(Box::from("input must be a valid ascii code"));
                }
                let result = number as char;
                Ok(Token::Word(result.to_string()))
            },
        )
    }

    pub fn list() -> Self {
        Command::reserved(
            "list",
            Params::Variadic(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let mut result = decode_word(com, &args, 0)?;
                for index in 1..args.len() {
                    let word = decode_word(com, &args, index)?;
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
                    _ => Err(Box::from("must be word or list to get count")),
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
                            Err(Box::from("list index out of bounds"))
                        }
                    }
                    Token::Word(word) => {
                        if let Some(chr) = word.chars().nth(index) {
                            Ok(Token::Word(chr.to_string()))
                        } else {
                            Err(Box::from("word index out of bounds"))
                        }
                    }
                    _ => Err(Box::from("cannot get item")),
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
                    Err(Box::from("list is empty"))
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
                    Err(Box::from("list is empty"))
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
                    return Err(Box::from("list is empty"));
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
                    return Err(Box::from("list is empty"));
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
                let item = decode_word(com, &args, 0)?;
                let list = decode_list(com, &args, 1)?;
                let result = format!("{} {}", item, list);
                Ok(Token::List(result))
            },
        )
    }

    pub fn lput() -> Self {
        Command::reserved(
            "lput",
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let item = decode_word(com, &args, 0)?;
                let list = decode_list(com, &args, 1)?;
                let result = format!("{} {}", list, item);
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
                    _ => return Err(Box::from("type cannot be empty")),
                };
                Ok(Token::Boolean(result))
            },
        )
    }
}
