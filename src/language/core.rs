use crate::interpreter::interpreter::Interpreter;
use crate::language::command::Command;
use crate::language::command::Procedure;
use crate::language::token::Token;
use crate::language::util::decode_boolean;
use crate::language::util::decode_token;
use crate::language::util::{decode_list, decode_number, decode_proc, decode_string};

impl Command {
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

    pub fn paren() -> Self {
        Command {
            name: String::from("__paren"),
            params: vec![String::from("code")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let code = decode_list(args.get(0))?;
                let code_with_return = format!("op {}", code);
                int.interpret(&code_with_return)
            },
        }
    }

    pub fn repeat() -> Self {
        Command {
            name: String::from("repeat"),
            params: vec![String::from("count"), String::from("block")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let count = decode_number(args.get(0))? as usize;
                let code = decode_list(args.get(1))?;
                let local_params = vec![(String::from("__loopcount"), Token::Number(count as f32))];
                let looping_code = code + "\n__loopback";
                int.execute_code_in_new_scope(&looping_code, local_params)?;
                Ok(Token::Void)
            },
        }
    }

    pub fn forever() -> Self {
        Command {
            name: String::from("forever"),
            params: vec![String::from("block")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let code = decode_list(args.get(0))?;
                let local_params = vec![];
                let looping_code = code + "\n__loopback";
                int.execute_code_in_new_scope(&looping_code, local_params)?;
                Ok(Token::Void)
            },
        }
    }

    pub fn loopback() -> Self {
        Command {
            name: String::from("__loopback"),
            params: vec![],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let loop_var_name = String::from("__loopcount");
                let count_token = int.datastore.get_variable(&loop_var_name);
                if let Some(Token::Number(count)) = count_token {
                    if *count > 0.0 {
                        int.lexer.return_to_start_of_top_frame();
                        int.datastore
                            .set_variable(loop_var_name, Token::Number(count - 1.0));
                    }
                } else {
                    int.lexer.return_to_start_of_top_frame();
                }
                Ok(Token::Void)
            },
        }
    }

    pub fn readchar() -> Self {
        Command {
            name: String::from("readchar"),
            params: vec![],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                if let Some(key) = int.datastore.get_one_key() {
                    Ok(Token::String(key))
                } else {
                    Ok(Token::String(String::from("")))
                }
            },
        }
    }

    pub fn ask() -> Self {
        Command {
            name: String::from("ask"),
            params: vec![String::from("turtle"), String::from("property")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let name = decode_string(args.get(0))?;
                let property = decode_string(args.get(1))?;
                if let Some(turtle) = int.datastore.get_turtle(&name) {
                    let token = match property.as_str() {
                        "x" => Token::Number(turtle.pos.0),
                        "y" => Token::Number(turtle.pos.1),
                        "pos" => Token::List(format!("{} {}", turtle.pos.0, turtle.pos.1)),
                        "heading" => Token::Number(turtle.heading),
                        "color" => Token::Number(turtle.color),
                        _ => Token::Void,
                    };
                    return Ok(token);
                }
                Ok(Token::Void)
            },
        }
    }
}
