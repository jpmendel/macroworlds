use crate::interpreter::interpreter::Interpreter;
use crate::language::command::Command;
use crate::language::event::UiEvent;
use crate::language::token::Token;
use crate::language::util::{decode_list, decode_number, decode_string};
use std::thread;
use std::time::Duration;

impl Command {
    pub fn fd() -> Self {
        Command {
            name: String::from("fd"),
            params: vec![String::from("dist")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let dist = decode_number(args.get(0))?;
                int.emit_ui_event(UiEvent::Fd(dist));
                Ok(Token::Void)
            },
        }
    }

    pub fn bk() -> Self {
        Command {
            name: String::from("bk"),
            params: vec![String::from("dist")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let dist = decode_number(args.get(0))?;
                int.emit_ui_event(UiEvent::Bk(dist));
                Ok(Token::Void)
            },
        }
    }

    pub fn lt() -> Self {
        Command {
            name: String::from("lt"),
            params: vec![String::from("angle")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let angle = decode_number(args.get(0))?;
                int.emit_ui_event(UiEvent::Lt(angle));
                Ok(Token::Void)
            },
        }
    }

    pub fn rt() -> Self {
        Command {
            name: String::from("rt"),
            params: vec![String::from("angle")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let angle = decode_number(args.get(0))?;
                int.emit_ui_event(UiEvent::Rt(angle));
                Ok(Token::Void)
            },
        }
    }

    pub fn setpos() -> Self {
        Command {
            name: String::from("setpos"),
            params: vec![String::from("coords")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let list = decode_list(args.get(0))?;
                let coords: Vec<&str> = list.split(' ').collect();
                if coords.len() != 2 {
                    return Err(Box::from("invalid coordinates"));
                }
                let x = coords[0].parse::<f32>()?;
                let y = coords[1].parse::<f32>()?;
                int.emit_ui_event(UiEvent::Setpos(x, y));
                Ok(Token::Void)
            },
        }
    }

    pub fn seth() -> Self {
        Command {
            name: String::from("seth"),
            params: vec![String::from("angle")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let heading = decode_number(args.get(0))?;
                int.emit_ui_event(UiEvent::Seth(heading));
                Ok(Token::Void)
            },
        }
    }

    pub fn setc() -> Self {
        Command {
            name: String::from("setc"),
            params: vec![String::from("color")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let color = decode_number(args.get(0))?;
                int.emit_ui_event(UiEvent::Setc(color));
                Ok(Token::Void)
            },
        }
    }

    pub fn pd() -> Self {
        Command {
            name: String::from("pd"),
            params: vec![],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                int.emit_ui_event(UiEvent::Pd);
                Ok(Token::Void)
            },
        }
    }

    pub fn pu() -> Self {
        Command {
            name: String::from("pu"),
            params: vec![],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                int.emit_ui_event(UiEvent::Pu);
                Ok(Token::Void)
            },
        }
    }

    pub fn st() -> Self {
        Command {
            name: String::from("st"),
            params: vec![],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                int.emit_ui_event(UiEvent::St);
                Ok(Token::Void)
            },
        }
    }

    pub fn ht() -> Self {
        Command {
            name: String::from("ht"),
            params: vec![],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                int.emit_ui_event(UiEvent::Ht);
                Ok(Token::Void)
            },
        }
    }

    pub fn clean() -> Self {
        Command {
            name: String::from("clean"),
            params: vec![],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                int.emit_ui_event(UiEvent::Clean);
                Ok(Token::Void)
            },
        }
    }

    pub fn newturtle() -> Self {
        Command {
            name: String::from("newturtle"),
            params: vec![String::from("name")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let name = decode_string(args.get(0))?;
                int.emit_ui_event(UiEvent::Newturtle(name));
                Ok(Token::Void)
            },
        }
    }

    pub fn addr() -> Self {
        Command {
            name: String::from("addr"),
            params: vec![String::from("arg")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let arg = decode_string(args.get(0))?;
                int.emit_ui_event(UiEvent::Addr(arg));
                Ok(Token::Void)
            },
        }
    }

    pub fn wait() -> Self {
        Command {
            name: String::from("wait"),
            params: vec![String::from("duration")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let duration = decode_number(args.get(0))? as u64;
                int.emit_ui_event(UiEvent::Wait(duration.clone()));
                thread::sleep(Duration::from_millis(duration));
                Ok(Token::Void)
            },
        }
    }
}
