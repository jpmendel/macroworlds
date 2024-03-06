use crate::interpreter::interpreter::Interpreter;
use crate::language::command::Command;
use crate::language::command::Procedure;
use crate::language::token::Token;
use crate::language::util::decode_boolean;
use crate::language::util::decode_token;
use crate::language::util::{decode_list, decode_number, decode_proc, decode_string};

impl Command {
    pub fn repeat() -> Self {
        Command {
            name: String::from("repeat"),
            params: vec![String::from("count"), String::from("block")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let count = decode_number(args.get(0))? as usize;
                let code = decode_list(args.get(1))?;
                for _ in 0..count {
                    int.interpret(&code)?;
                }
                Ok(Token::Void)
            },
        }
    }

    pub fn make() -> Self {
        Command {
            name: String::from("make"),
            params: vec![String::from("var"), String::from("value")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let name = decode_string(args.get(0))?;
                let value = decode_token(args.get(1))?.clone();
                int.datastore.set_variable(name, value);
                Ok(Token::Void)
            },
        }
    }

    pub fn to() -> Self {
        Command {
            name: String::from("to"),
            params: vec![String::from("proc")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let (name, params, code) = decode_proc(args.get(0))?;
                int.define_procedure(Procedure { name, params, code });
                Ok(Token::Void)
            },
        }
    }

    pub fn local() -> Self {
        Command {
            name: String::from("local"),
            params: vec![String::from("variable")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let name = decode_string(args.get(0))?;
                int.datastore.set_local(name);
                Ok(Token::Void)
            },
        }
    }

    pub fn op() -> Self {
        Command {
            name: String::from("op"),
            params: vec![String::from("value")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let return_value = decode_token(args.get(0))?;
                Ok(return_value.clone())
            },
        }
    }

    pub fn ifthen() -> Self {
        Command {
            name: String::from("if"),
            params: vec![String::from("condition"), String::from("true_code")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let condition = decode_boolean(args.get(0))?;
                let true_code = decode_list(args.get(1))?;
                if condition {
                    int.interpret(&true_code)?;
                }
                Ok(Token::Void)
            },
        }
    }
    pub fn ifelse() -> Self {
        Command {
            name: String::from("ifelse"),
            params: vec![
                String::from("condition"),
                String::from("true_code"),
                String::from("false_code"),
            ],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let condition = decode_boolean(args.get(0))?;
                let true_code = decode_list(args.get(1))?;
                let false_code = decode_list(args.get(2))?;
                if condition {
                    int.interpret(&true_code)?;
                } else {
                    int.interpret(&false_code)?;
                }
                Ok(Token::Void)
            },
        }
    }
}
