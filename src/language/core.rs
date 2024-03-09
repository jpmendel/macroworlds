use crate::interpreter::interpreter::Interpreter;
use crate::language::command::{Command, Params, Procedure};
use crate::language::token::Token;
use crate::language::util::{decode_proc, decode_token, decode_word, join_to_list_string};

impl Command {
    pub fn make() -> Self {
        Command {
            name: String::from("make"),
            params: Params::Fixed(2),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let name = decode_word(args.get(0))?;
                let token = decode_token(args.get(1))?.clone();
                let value = match token {
                    Token::List(list) => {
                        let tokens = int.parse_list(&list)?;
                        let joined = join_to_list_string(tokens);
                        Token::List(joined)
                    }
                    token => token,
                };
                int.state.set_variable(name, value);
                Ok(Token::Void)
            },
        }
    }

    pub fn to() -> Self {
        Command {
            name: String::from("to"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let (name, params, code) = decode_proc(args.get(0))?;
                int.define_procedure(Procedure { name, params, code });
                Ok(Token::Void)
            },
        }
    }

    pub fn local() -> Self {
        Command {
            name: String::from("local"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let name = decode_word(args.get(0))?;
                int.state.set_local(name);
                Ok(Token::Void)
            },
        }
    }

    pub fn op() -> Self {
        Command {
            name: String::from("op"),
            params: Params::Fixed(1),
            action: |_int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let return_value = decode_token(args.get(0))?;
                Ok(return_value.clone())
            },
        }
    }

    pub fn who() -> Self {
        Command {
            name: String::from("who"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.current_turtle()?;
                Ok(Token::Word(turtle.name.clone()))
            },
        }
    }

    pub fn tto() -> Self {
        Command {
            name: String::from("tto"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let name = decode_word(args.get(0))?;
                let success = int.state.set_current_object(name.clone());
                if !success {
                    return Err(Box::from(format!("no turtle named {}", name)));
                }
                Ok(Token::Void)
            },
        }
    }

    pub fn ask() -> Self {
        Command {
            name: String::from("ask"),
            params: Params::Fixed(2),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let name = decode_word(args.get(0))?;
                let token = decode_token(args.get(1))?;
                match token {
                    Token::Word(word) => {
                        let code = format!("op {}", word);
                        let token = int.interpret(&code)?;
                        Ok(token)
                    }
                    Token::List(list) => {
                        let current_obj_name = int.state.current_object()?.name().clone();
                        let success = int.state.set_current_object(name.clone());
                        if success {
                            let _ = int.interpret(&list);
                            int.state.set_current_object(current_obj_name);
                            Ok(Token::Void)
                        } else {
                            Err(Box::from(format!("no turtle named {}", name)))
                        }
                    }
                    _ => Err(Box::from("expected word or list as input")),
                }
            },
        }
    }

    pub fn key() -> Self {
        Command {
            name: String::from("key?"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let has_key = int.state.has_key();
                Ok(Token::Boolean(has_key))
            },
        }
    }

    pub fn readchar() -> Self {
        Command {
            name: String::from("readchar"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                if let Some(key) = int.state.get_one_key() {
                    Ok(Token::Word(key))
                } else {
                    Ok(Token::Word(String::from("")))
                }
            },
        }
    }
}
