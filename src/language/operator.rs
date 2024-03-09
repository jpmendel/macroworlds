use crate::interpreter::interpreter::Interpreter;
use crate::language::command::{Command, Params};
use crate::language::token::Token;
use crate::language::util::{
    are_tokens_equal, decode_boolean, decode_list, decode_number, decode_token, decode_word,
    join_to_list_string,
};

impl Command {
    pub fn sum() -> Self {
        Command {
            name: String::from("sum"),
            params: Params::Variadic(2),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let mut result = 0.0;
                for arg in &args {
                    let num = decode_number(Some(arg))?;
                    result += num;
                }
                Ok(Token::Number(result))
            },
        }
    }

    pub fn difference() -> Self {
        Command {
            name: String::from("difference"),
            params: Params::Fixed(2),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let num1 = decode_number(args.get(0))?;
                let num2 = decode_number(args.get(1))?;
                let result = num1 - num2;
                Ok(Token::Number(result))
            },
        }
    }

    pub fn product() -> Self {
        Command {
            name: String::from("product"),
            params: Params::Variadic(2),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let mut result = 0.0;
                for arg in &args {
                    let num = decode_number(Some(arg))?;
                    result *= num;
                }
                Ok(Token::Number(result))
            },
        }
    }

    pub fn quotient() -> Self {
        Command {
            name: String::from("quotient"),
            params: Params::Fixed(2),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let num1 = decode_number(args.get(0))?;
                let num2 = decode_number(args.get(1))?;
                if num2 == 0.0 {
                    return Err(Box::from("cannot divide by zero"));
                }
                let result = num1 / num2;
                Ok(Token::Number(result))
            },
        }
    }

    pub fn power() -> Self {
        Command {
            name: String::from("power"),
            params: Params::Fixed(2),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let num1 = decode_number(args.get(0))?;
                let num2 = decode_number(args.get(1))?;
                let result = num1.powf(num2);
                Ok(Token::Number(result))
            },
        }
    }

    pub fn minus() -> Self {
        Command {
            name: String::from("minus"),
            params: Params::Fixed(1),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let number = decode_number(args.get(0))?;
                let result = -number;
                Ok(Token::Number(result))
            },
        }
    }

    pub fn equal() -> Self {
        Command {
            name: String::from("equal?"),
            params: Params::Fixed(2),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let arg1 = decode_token(args.get(0))?;
                let arg2 = decode_token(args.get(1))?;
                let result = are_tokens_equal(arg1, arg2);
                Ok(Token::Boolean(result))
            },
        }
    }

    pub fn greater() -> Self {
        Command {
            name: String::from("greater?"),
            params: Params::Fixed(2),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let arg1 = decode_number(args.get(0))?;
                let arg2 = decode_number(args.get(1))?;
                let result = arg1 > arg2;
                Ok(Token::Boolean(result))
            },
        }
    }

    pub fn less() -> Self {
        Command {
            name: String::from("less?"),
            params: Params::Fixed(2),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let arg1 = decode_number(args.get(0))?;
                let arg2 = decode_number(args.get(1))?;
                let result = arg1 < arg2;
                Ok(Token::Boolean(result))
            },
        }
    }

    pub fn or() -> Self {
        Command {
            name: String::from("or"),
            params: Params::Variadic(2),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let mut result = decode_boolean(args.get(0))?;
                for arg in &args[1..] {
                    let bool = decode_boolean(Some(arg))?;
                    result = result || bool;
                }
                Ok(Token::Boolean(result))
            },
        }
    }

    pub fn and() -> Self {
        Command {
            name: String::from("and"),
            params: Params::Variadic(2),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let mut result = decode_boolean(args.get(0))?;
                for arg in &args[1..] {
                    let bool = decode_boolean(Some(arg))?;
                    result = result && bool;
                }
                Ok(Token::Boolean(result))
            },
        }
    }

    pub fn not() -> Self {
        Command {
            name: String::from("not"),
            params: Params::Fixed(1),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let boolean = decode_boolean(args.get(0))?;
                let result = !boolean;
                Ok(Token::Boolean(result))
            },
        }
    }

    pub fn word() -> Self {
        Command {
            name: String::from("word"),
            params: Params::Variadic(2),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let mut result = decode_word(args.get(0))?;
                for arg in &args[1..] {
                    let word = decode_word(Some(arg))?;
                    result += &word;
                }
                Ok(Token::Word(result))
            },
        }
    }

    pub fn list() -> Self {
        Command {
            name: String::from("list"),
            params: Params::Variadic(2),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let mut result = decode_word(args.get(0))?;
                for arg in &args[1..] {
                    let word = decode_word(Some(arg))?;
                    result += &format!(" {}", word);
                }
                Ok(Token::List(result))
            },
        }
    }

    pub fn item() -> Self {
        Command {
            name: String::from("item"),
            params: Params::Fixed(2),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let index = decode_number(args.get(0))? as usize;
                let token = decode_token(args.get(1))?;
                match token {
                    Token::List(list) => {
                        let items = int.parse_list(list)?;
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
        }
    }

    pub fn first() -> Self {
        Command {
            name: String::from("first"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let list = decode_list(args.get(0))?;
                let items = int.parse_list(&list)?;
                if let Some(first) = items.first() {
                    Ok(first.clone())
                } else {
                    Err(Box::from("list is empty"))
                }
            },
        }
    }

    pub fn last() -> Self {
        Command {
            name: String::from("last"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let list = decode_list(args.get(0))?;
                let items = int.parse_list(&list)?;
                if let Some(last) = items.last() {
                    Ok(last.clone())
                } else {
                    Err(Box::from("list is empty"))
                }
            },
        }
    }

    pub fn butfirst() -> Self {
        Command {
            name: String::from("butfirst"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let list = decode_list(args.get(0))?;
                let items = int.parse_list(&list)?;
                if items.is_empty() {
                    return Err(Box::from("list is empty"));
                }
                let rest = &items[1..];
                let joined = join_to_list_string(rest.to_vec());
                Ok(Token::List(joined))
            },
        }
    }

    pub fn butlast() -> Self {
        Command {
            name: String::from("butlast"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let list = decode_list(args.get(0))?;
                let items = int.parse_list(&list)?;
                if items.is_empty() {
                    return Err(Box::from("list is empty"));
                }
                let rest = &items[..items.len() - 1];
                let joined = join_to_list_string(rest.to_vec());
                Ok(Token::List(joined))
            },
        }
    }

    pub fn fput() -> Self {
        Command {
            name: String::from("fput"),
            params: Params::Fixed(2),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let item = decode_word(args.get(0))?;
                let list = decode_list(args.get(1))?;
                let result = format!("{} {}", item, list);
                Ok(Token::List(result))
            },
        }
    }

    pub fn lput() -> Self {
        Command {
            name: String::from("lput"),
            params: Params::Fixed(2),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let item = decode_word(args.get(0))?;
                let list = decode_list(args.get(1))?;
                let result = format!("{} {}", list, item);
                Ok(Token::List(result))
            },
        }
    }

    pub fn member() -> Self {
        Command {
            name: String::from("member?"),
            params: Params::Fixed(2),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let item = decode_word(args.get(0))?;
                let list = decode_list(args.get(1))?;
                let result = list.contains(&item);
                Ok(Token::Boolean(result))
            },
        }
    }

    pub fn empty() -> Self {
        Command {
            name: String::from("empty?"),
            params: Params::Fixed(1),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let token = decode_token(args.get(0))?;
                let result = match token {
                    Token::Word(word) => word.is_empty(),
                    Token::List(list) => list.is_empty(),
                    _ => return Err(Box::from("type cannot be empty")),
                };
                Ok(Token::Boolean(result))
            },
        }
    }
}
