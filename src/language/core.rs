use crate::interpreter::interpreter::Interpreter;
use crate::language::command::{Command, Params, Procedure};
use crate::language::token::Token;
use crate::language::util::{
    decode_number, decode_proc, decode_token, decode_word, join_to_list_string,
};
use rand::Rng;

impl Command {
    pub fn make() -> Self {
        Command::reserved(
            String::from("make"),
            Params::Fixed(2),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let name = decode_word(com, &args, 0)?;
                let token = decode_token(com, &args, 1)?;
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
        )
    }

    pub fn to() -> Self {
        Command::reserved(
            String::from("to"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let (name, params, code) = decode_proc(com, &args, 0)?;
                int.define_procedure(Procedure { name, params, code })?;
                Ok(Token::Void)
            },
        )
    }

    pub fn local() -> Self {
        Command::reserved(
            String::from("local"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let name = decode_word(com, &args, 0)?;
                int.state.set_local(name);
                Ok(Token::Void)
            },
        )
    }

    pub fn output() -> Self {
        Command::reserved(
            String::from("output"),
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let return_value = decode_token(com, &args, 0)?;
                Ok(return_value.clone())
            },
        )
    }

    pub fn who() -> Self {
        Command::reserved(
            String::from("who"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.current_turtle()?;
                Ok(Token::Word(turtle.name.clone()))
            },
        )
    }

    pub fn talkto() -> Self {
        Command::reserved(
            String::from("talkto"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let name = decode_word(com, &args, 0)?;
                let success = int.state.set_current_object(name.clone());
                if !success {
                    return Err(Box::from(format!("no turtle named {}", name)));
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn ask() -> Self {
        Command::reserved(
            String::from("ask"),
            Params::Fixed(2),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let name = decode_word(com, &args, 0)?;
                let token = decode_token(com, &args, 1)?;
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
        )
    }

    pub fn turtlesown() -> Self {
        Command::reserved(
            String::from("turtlesown"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let name = decode_word(com, &args, 0)?;
                int.define_object_property(name)?;
                Ok(Token::Void)
            },
        )
    }

    pub fn timer() -> Self {
        Command::reserved(
            String::from("timer"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let time = match int.state.get_time() {
                    Ok(time) => time,
                    Err(..) => return Err(Box::from("timer unable to get time")),
                };
                Ok(Token::Number(time as f32))
            },
        )
    }

    pub fn resett() -> Self {
        Command::reserved(
            String::from("resett"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                int.state.reset_timer();
                Ok(Token::Void)
            },
        )
    }

    pub fn key() -> Self {
        Command::reserved(
            String::from("key?"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let has_key = int.state.has_key();
                Ok(Token::Boolean(has_key))
            },
        )
    }

    pub fn readchar() -> Self {
        Command::reserved(
            String::from("readchar"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                if let Some(key) = int.state.get_one_key() {
                    Ok(Token::Word(key))
                } else {
                    Ok(Token::Word(String::from("")))
                }
            },
        )
    }

    pub fn random() -> Self {
        Command::reserved(
            String::from("random"),
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let max = decode_number(com, &args, 0)? as u32;
                let random = rand::thread_rng().gen_range(0..max);
                Ok(Token::Number(random as f32))
            },
        )
    }

    pub fn pick() -> Self {
        Command::reserved(
            String::from("pick"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let token = decode_token(com, &args, 0)?;
                match token {
                    Token::Word(word) => {
                        let random = rand::thread_rng().gen_range(0..word.len());
                        let chr = word.chars().nth(random).unwrap().to_string();
                        Ok(Token::Word(chr))
                    }
                    Token::List(list) => {
                        let items = int.parse_list(&list)?;
                        let random = rand::thread_rng().gen_range(0..items.len());
                        let item = items.get(random).unwrap().clone();
                        Ok(item)
                    }
                    _ => Err(Box::from("expected a word or list")),
                }
            },
        )
    }

    pub fn pi() -> Self {
        Command::reserved(
            String::from("pi"),
            Params::None,
            |_int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let pi = std::f32::consts::PI;
                Ok(Token::Number(pi))
            },
        )
    }
}
