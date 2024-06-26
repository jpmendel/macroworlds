use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::language::structure::{Command, Params};
use crate::interpreter::language::token::{Token, TokenVec};
use crate::interpreter::language::util::decode;
use crate::interpreter::language::util::io::KeyCode;
use rand::Rng;

impl Command {
    pub fn make() -> Self {
        Command::reserved(
            "make",
            Params::Fixed(2),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let name = decode::word(com, &args, 0)?;
                let token = decode::token(com, &args, 1)?;
                let value = match token {
                    Token::List(list) => {
                        let tokens = int.parse_list(&list, false)?;
                        let joined = tokens.join_to_list_string();
                        Token::List(joined)
                    }
                    token => token,
                };
                int.state.data.set_variable(&name, value);
                Ok(Token::Void)
            },
        )
    }

    pub fn to() -> Self {
        Command::reserved(
            "to",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let proc = decode::procedure(com, &args, 0)?;
                int.define_procedure(proc)?;
                Ok(Token::Void)
            },
        )
    }

    pub fn local() -> Self {
        Command::reserved(
            "local",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let name = decode::word(com, &args, 0)?;
                int.state.data.init_local(&name, Token::Void);
                Ok(Token::Void)
            },
        )
    }

    pub fn letvar() -> Self {
        Command::reserved(
            "let",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let list = decode::list(com, &args, 0)?;
                let list_items = int.parse_list(&list, true)?;
                for index in (0..list_items.len()).step_by(2) {
                    if let Some(Token::Word(name)) = list_items.get(index) {
                        if let Some(value) = list_items.get(index + 1) {
                            int.state.data.init_local(&name, value.clone());
                        }
                    }
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn clearname() -> Self {
        Command::reserved(
            "clearname",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let token = decode::token(com, &args, 0)?;
                match token {
                    Token::Word(word) => int.state.data.remove_variable(&word),
                    Token::List(list) => {
                        let list_items = int.parse_list(&list, false)?;
                        for item in list_items {
                            match item {
                                Token::Word(word) => int.state.data.remove_variable(&word),
                                _ => return Err(Box::from("clearname expected a list of words")),
                            }
                        }
                    }
                    _ => return Err(Box::from("clearname expected a word or list as input")),
                };
                Ok(Token::Void)
            },
        )
    }

    pub fn clearnames() -> Self {
        Command::reserved(
            "clearnames",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                int.state.data.remove_all_variables_in_scope();
                Ok(Token::Void)
            },
        )
    }

    pub fn output() -> Self {
        Command::reserved(
            "output",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let return_value = decode::token(com, &args, 0)?;
                Ok(return_value.clone())
            },
        )
    }

    pub fn run() -> Self {
        Command::reserved(
            "run",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let code = match decode::token(com, &args, 0)? {
                    Token::Word(word) => word,
                    Token::List(list) => list,
                    _ => return Err(Box::from("run expected a word or list as input")),
                };
                int.interpret(&code)?;
                Ok(Token::Void)
            },
        )
    }

    pub fn procedures() -> Self {
        Command::reserved(
            "procedures",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let procs = int.state.data.get_all_procedures();
                let mut list_string = String::new();
                for index in 0..procs.len() {
                    if index == 0 {
                        list_string += &format!("{}", procs[index].name);
                    } else {
                        list_string += &format!(" {}", procs[index].name);
                    }
                }
                Ok(Token::List(list_string))
            },
        )
    }

    pub fn who() -> Self {
        Command::reserved(
            "who",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let object = int.state.canvas.current_object()?;
                Ok(Token::Word(object.name().to_string()))
            },
        )
    }

    pub fn talkto() -> Self {
        Command::reserved(
            "talkto",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let name = decode::word(com, &args, 0)?;
                let success = int.state.canvas.set_current_object(&name);
                if !success {
                    return Err(Box::from(format!("talkto found no object named {}", name)));
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn ask() -> Self {
        Command::reserved(
            "ask",
            Params::Fixed(2),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let name = decode::word(com, &args, 0)?;
                let token = decode::token(com, &args, 1)?;
                match token {
                    Token::Word(word) => {
                        let code = format!("op {}", word);
                        let token = int.interpret(&code)?;
                        Ok(token)
                    }
                    Token::List(list) => {
                        let current_obj_name =
                            int.state.canvas.current_object()?.name().to_string();
                        let success = int.state.canvas.set_current_object(&name);
                        if success {
                            let _ = int.interpret(&list);
                            int.state.canvas.set_current_object(&current_obj_name);
                            Ok(Token::Void)
                        } else {
                            Err(Box::from(format!("no object named {}", name)))
                        }
                    }
                    _ => Err(Box::from("ask expected a word or list as input")),
                }
            },
        )
    }

    pub fn turtlesown() -> Self {
        Command::reserved(
            "turtlesown",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let name = decode::word(com, &args, 0)?;
                int.define_object_property(&name)?;
                Ok(Token::Void)
            },
        )
    }

    pub fn timer() -> Self {
        Command::reserved(
            "timer",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
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
            "resett",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                int.state.reset_timer();
                Ok(Token::Void)
            },
        )
    }

    pub fn key() -> Self {
        Command::reserved(
            "key?",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let has_key = int.state.input.has_key();
                Ok(Token::Boolean(has_key))
            },
        )
    }

    pub fn readchar() -> Self {
        Command::reserved(
            "readchar",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                if let Some(key) = int.state.input.get_one_key() {
                    Ok(Token::Word(key))
                } else {
                    Ok(Token::Word(String::new()))
                }
            },
        )
    }

    pub fn keydown() -> Self {
        Command::reserved(
            "keydown?",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let token = decode::token(com, &args, 0)?;
                let is_down = match token {
                    Token::Word(word) => int.state.input.is_key_down(&word),
                    Token::Number(number) => {
                        let key = (number as u8).to_key_name()?;
                        int.state.input.is_key_down(&key)
                    }
                    _ => {
                        return Err(Box::from("keydown? expected key name or ascii as input"));
                    }
                };
                Ok(Token::Boolean(is_down))
            },
        )
    }

    pub fn clicked() -> Self {
        Command::reserved(
            "clicked?",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let has_click = int.state.input.has_click();
                Ok(Token::Boolean(has_click))
            },
        )
    }

    pub fn readclick() -> Self {
        Command::reserved(
            "readclick",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                if let Some(click) = int.state.input.get_one_click() {
                    Ok(Token::List(format!("{} {}", click.x, click.y)))
                } else {
                    Ok(Token::List(String::new()))
                }
            },
        )
    }

    pub fn random() -> Self {
        Command::reserved(
            "random",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let max = decode::number(com, &args, 0)? as u32;
                let random = rand::thread_rng().gen_range(0..max);
                Ok(Token::Number(random as f32))
            },
        )
    }

    pub fn pick() -> Self {
        Command::reserved(
            "pick",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let token = decode::token(com, &args, 0)?;
                match token {
                    Token::Word(word) => {
                        let random = rand::thread_rng().gen_range(0..word.len());
                        let chr = word.chars().nth(random).unwrap().to_string();
                        Ok(Token::Word(chr))
                    }
                    Token::List(list) => {
                        let items = int.parse_list(&list, true)?;
                        let random = rand::thread_rng().gen_range(0..items.len());
                        let item = items.get(random).unwrap().clone();
                        Ok(item)
                    }
                    _ => Err(Box::from("expected a word or list")),
                }
            },
        )
    }
}
