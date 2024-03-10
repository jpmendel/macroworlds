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
            String::from("sum"),
            Params::Variadic(2),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let mut result = 0.0;
                for index in 0..args.len() {
                    let num = decode_number(com, &args, index)?;
                    result += num;
                }
                Ok(Token::Number(result))
            },
        )
    }

    pub fn difference() -> Self {
        Command::reserved(
            String::from("difference"),
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let num1 = decode_number(com, &args, 0)?;
                let num2 = decode_number(com, &args, 1)?;
                let result = num1 - num2;
                Ok(Token::Number(result))
            },
        )
    }

    pub fn product() -> Self {
        Command::reserved(
            String::from("product"),
            Params::Variadic(2),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let mut result = 0.0;
                for index in 0..args.len() {
                    let num = decode_number(com, &args, index)?;
                    result *= num;
                }
                Ok(Token::Number(result))
            },
        )
    }

    pub fn quotient() -> Self {
        Command::reserved(
            String::from("quotient"),
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
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
            String::from("remainder"),
            Params::Variadic(2),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let num1 = decode_number(com, &args, 0)?;
                let num2 = decode_number(com, &args, 1)?;
                let result = num1 % num2;
                Ok(Token::Number(result))
            },
        )
    }

    pub fn power() -> Self {
        Command::reserved(
            String::from("power"),
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let num1 = decode_number(com, &args, 0)?;
                let num2 = decode_number(com, &args, 1)?;
                let result = num1.powf(num2);
                Ok(Token::Number(result))
            },
        )
    }

    pub fn sqrt() -> Self {
        Command::reserved(
            String::from("sqrt"),
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
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
            String::from("minus"),
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let number = decode_number(com, &args, 0)?;
                let result = -number;
                Ok(Token::Number(result))
            },
        )
    }

    pub fn abs() -> Self {
        Command::reserved(
            String::from("abs"),
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let number = decode_number(com, &args, 0)?;
                let result = number.abs();
                Ok(Token::Number(result))
            },
        )
    }

    pub fn equal() -> Self {
        Command::reserved(
            String::from("equal?"),
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let arg1 = decode_token(com, &args, 0)?;
                let arg2 = decode_token(com, &args, 1)?;
                let result = are_tokens_equal(&arg1, &arg2);
                Ok(Token::Boolean(result))
            },
        )
    }

    pub fn greater() -> Self {
        Command::reserved(
            String::from("greater?"),
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let arg1 = decode_number(com, &args, 0)?;
                let arg2 = decode_number(com, &args, 1)?;
                let result = arg1 > arg2;
                Ok(Token::Boolean(result))
            },
        )
    }

    pub fn less() -> Self {
        Command::reserved(
            String::from("less?"),
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let arg1 = decode_number(com, &args, 0)?;
                let arg2 = decode_number(com, &args, 1)?;
                let result = arg1 < arg2;
                Ok(Token::Boolean(result))
            },
        )
    }

    pub fn or() -> Self {
        Command::reserved(
            String::from("or"),
            Params::Variadic(2),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
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
            String::from("and"),
            Params::Variadic(2),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
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
            String::from("not"),
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let boolean = decode_boolean(com, &args, 0)?;
                let result = !boolean;
                Ok(Token::Boolean(result))
            },
        )
    }

    pub fn int() -> Self {
        Command::reserved(
            String::from("int"),
            Params::Variadic(1),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let number = decode_number(com, &args, 0)?;
                Ok(Token::Number(number.floor()))
            },
        )
    }

    pub fn word() -> Self {
        Command::reserved(
            String::from("word"),
            Params::Variadic(2),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
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
            String::from("ascii"),
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
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

    pub fn list() -> Self {
        Command::reserved(
            String::from("list"),
            Params::Variadic(2),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let mut result = decode_word(com, &args, 0)?;
                for index in 1..args.len() {
                    let word = decode_word(com, &args, index)?;
                    result += &format!(" {}", word);
                }
                Ok(Token::List(result))
            },
        )
    }

    pub fn item() -> Self {
        Command::reserved(
            String::from("item"),
            Params::Fixed(2),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let index = decode_number(com, &args, 0)? as usize;
                let token = decode_token(com, &args, 1)?;
                match token {
                    Token::List(list) => {
                        let items = int.parse_list(&list)?;
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
            String::from("first"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let list = decode_list(com, &args, 0)?;
                let items = int.parse_list(&list)?;
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
            String::from("last"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let list = decode_list(com, &args, 0)?;
                let items = int.parse_list(&list)?;
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
            String::from("butfirst"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let list = decode_list(com, &args, 0)?;
                let items = int.parse_list(&list)?;
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
            String::from("butlast"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let list = decode_list(com, &args, 0)?;
                let items = int.parse_list(&list)?;
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
            String::from("fput"),
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let item = decode_word(com, &args, 0)?;
                let list = decode_list(com, &args, 1)?;
                let result = format!("{} {}", item, list);
                Ok(Token::List(result))
            },
        )
    }

    pub fn lput() -> Self {
        Command::reserved(
            String::from("lput"),
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let item = decode_word(com, &args, 0)?;
                let list = decode_list(com, &args, 1)?;
                let result = format!("{} {}", list, item);
                Ok(Token::List(result))
            },
        )
    }

    pub fn member() -> Self {
        Command::reserved(
            String::from("member?"),
            Params::Fixed(2),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let item = decode_word(com, &args, 0)?;
                let list = decode_list(com, &args, 1)?;
                let result = list.contains(&item);
                Ok(Token::Boolean(result))
            },
        )
    }

    pub fn empty() -> Self {
        Command::reserved(
            String::from("empty?"),
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
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
