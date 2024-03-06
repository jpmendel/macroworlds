use crate::interpreter::interpreter::Interpreter;
use crate::language::command::Command;
use crate::language::token::Token;
use crate::language::util::{decode_boolean, decode_number, decode_token};

use super::util::are_tokens_equal;

impl Command {
    pub fn add() -> Self {
        Command {
            name: String::from("add"),
            params: vec![String::from("arg1"), String::from("arg2")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let arg1 = decode_number(args.get(0))?;
                let arg2 = decode_number(args.get(1))?;
                let sum = arg1 + arg2;
                Ok(Token::Number(sum))
            },
        }
    }

    pub fn sub() -> Self {
        Command {
            name: String::from("sub"),
            params: vec![String::from("arg1"), String::from("arg2")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let arg1 = decode_number(args.get(0))?;
                let arg2 = decode_number(args.get(1))?;
                let diff = arg1 + arg2;
                Ok(Token::Number(diff))
            },
        }
    }

    pub fn mul() -> Self {
        Command {
            name: String::from("mul"),
            params: vec![String::from("arg1"), String::from("arg2")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let arg1 = decode_number(args.get(0))?;
                let arg2 = decode_number(args.get(1))?;
                let product = arg1 * arg2;
                Ok(Token::Number(product))
            },
        }
    }

    pub fn div() -> Self {
        Command {
            name: String::from("div"),
            params: vec![String::from("arg1"), String::from("arg2")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let arg1 = decode_number(args.get(0))?;
                let arg2 = decode_number(args.get(1))?;
                let quotient = arg1 / arg2;
                Ok(Token::Number(quotient))
            },
        }
    }

    pub fn eq() -> Self {
        Command {
            name: String::from("eq"),
            params: vec![String::from("arg1"), String::from("arg2")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let arg1 = decode_token(args.get(0))?;
                let arg2 = decode_token(args.get(1))?;
                let result = are_tokens_equal(arg1, arg2);
                Ok(Token::Boolean(result))
            },
        }
    }

    pub fn ne() -> Self {
        Command {
            name: String::from("ne"),
            params: vec![String::from("arg1"), String::from("arg2")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let arg1 = decode_token(args.get(0))?;
                let arg2 = decode_token(args.get(1))?;
                let result = !are_tokens_equal(arg1, arg2);
                Ok(Token::Boolean(result))
            },
        }
    }

    pub fn greater() -> Self {
        Command {
            name: String::from("greater"),
            params: vec![String::from("arg1"), String::from("arg2")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let arg1 = decode_number(args.get(0))?;
                let arg2 = decode_number(args.get(1))?;
                let result = arg1 > arg2;
                Ok(Token::Boolean(result))
            },
        }
    }

    pub fn less() -> Self {
        Command {
            name: String::from("less"),
            params: vec![String::from("arg1"), String::from("arg2")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let arg1 = decode_number(args.get(0))?;
                let arg2 = decode_number(args.get(1))?;
                let result = arg1 < arg2;
                Ok(Token::Boolean(result))
            },
        }
    }

    pub fn geq() -> Self {
        Command {
            name: String::from("geq"),
            params: vec![String::from("arg1"), String::from("arg2")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let arg1 = decode_number(args.get(0))?;
                let arg2 = decode_number(args.get(1))?;
                let result = arg1 >= arg2;
                Ok(Token::Boolean(result))
            },
        }
    }

    pub fn leq() -> Self {
        Command {
            name: String::from("leq"),
            params: vec![String::from("arg1"), String::from("arg2")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let arg1 = decode_number(args.get(0))?;
                let arg2 = decode_number(args.get(1))?;
                let result = arg1 <= arg2;
                Ok(Token::Boolean(result))
            },
        }
    }

    pub fn not() -> Self {
        Command {
            name: String::from("not"),
            params: vec![String::from("arg")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let boolean = decode_boolean(args.get(0))?;
                let result = !boolean;
                Ok(Token::Boolean(result))
            },
        }
    }
}
