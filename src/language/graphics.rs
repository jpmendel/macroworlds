use crate::interpreter::interpreter::Interpreter;
use crate::language::command::Command;
use crate::language::event::UiEvent;
use crate::language::token::Token;
use crate::language::util::{decode_list, decode_number, decode_string, decode_token};
use std::thread;
use std::time::Duration;

impl Command {
    pub fn fd() -> Self {
        Command {
            name: String::from("fd"),
            params: vec![String::from("dist")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let dist = decode_number(args.get(0))?;
                let turtle = int.datastore.current_turtle();
                let original_pos = turtle.pos.clone();
                let x = dist * turtle.heading.to_radians().cos();
                let y = dist * turtle.heading.to_radians().sin();
                let new_pos = (original_pos.0 + x, original_pos.1 + y);
                turtle.pos = new_pos.clone();
                let _ = int
                    .ui_sender
                    .send(UiEvent::TurtlePos(turtle.name.clone(), turtle.pos.clone()));

                if turtle.is_drawing {
                    let color = turtle.color.clone();
                    let line = int.datastore.add_line(original_pos, new_pos, color);
                    let _ = int.ui_sender.send(UiEvent::AddLine(line.clone()));
                }
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
                let turtle = int.datastore.current_turtle();
                let original_pos = turtle.pos.clone();
                let x = -dist * turtle.heading.to_radians().cos();
                let y = -dist * turtle.heading.to_radians().sin();
                let new_pos = (original_pos.0 + x, original_pos.1 + y);
                turtle.pos = new_pos.clone();
                let _ = int
                    .ui_sender
                    .send(UiEvent::TurtlePos(turtle.name.clone(), turtle.pos.clone()));

                if turtle.is_drawing {
                    let color = turtle.color.clone();
                    let line = int.datastore.add_line(original_pos, new_pos, color);
                    let _ = int.ui_sender.send(UiEvent::AddLine(line.clone()));
                }
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
                let turtle = int.datastore.current_turtle();
                turtle.heading -= angle;
                let _ = int.ui_sender.send(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
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
                let turtle = int.datastore.current_turtle();
                turtle.heading += angle;
                let _ = int.ui_sender.send(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
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
                let turtle = int.datastore.current_turtle();
                turtle.pos = (x, y);
                let _ = int.ui_sender.send(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
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
                let turtle = int.datastore.current_turtle();
                turtle.heading = heading;
                let _ = int.ui_sender.send(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
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
                let turtle = int.datastore.current_turtle();
                turtle.color = color;
                let _ = int.ui_sender.send(UiEvent::TurtleColor(
                    turtle.name.clone(),
                    turtle.color.clone(),
                ));
                Ok(Token::Void)
            },
        }
    }

    pub fn pd() -> Self {
        Command {
            name: String::from("pd"),
            params: vec![],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let turtle = int.datastore.current_turtle();
                turtle.is_drawing = true;
                Ok(Token::Void)
            },
        }
    }

    pub fn pu() -> Self {
        Command {
            name: String::from("pu"),
            params: vec![],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let turtle = int.datastore.current_turtle();
                turtle.is_drawing = false;
                Ok(Token::Void)
            },
        }
    }

    pub fn st() -> Self {
        Command {
            name: String::from("st"),
            params: vec![],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let turtle = int.datastore.current_turtle();
                turtle.is_visible = true;
                let _ = int
                    .ui_sender
                    .send(UiEvent::TurtleVisible(turtle.name.clone(), true));
                Ok(Token::Void)
            },
        }
    }

    pub fn ht() -> Self {
        Command {
            name: String::from("ht"),
            params: vec![],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let turtle = int.datastore.current_turtle();
                turtle.is_visible = false;
                let _ = int
                    .ui_sender
                    .send(UiEvent::TurtleVisible(turtle.name.clone(), false));
                Ok(Token::Void)
            },
        }
    }

    pub fn clean() -> Self {
        Command {
            name: String::from("clean"),
            params: vec![],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let _ = int.ui_sender.send(UiEvent::Clean);
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
                let turtle = int.datastore.create_turtle(name.clone());
                let _ = int.ui_sender.send(UiEvent::NewTurtle(name));
                Ok(Token::Void)
            },
        }
    }

    pub fn addr() -> Self {
        Command {
            name: String::from("addr"),
            params: vec![String::from("arg")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let name = decode_string(args.get(0))?;
                let success = int.datastore.set_current_turtle(&name);
                if !success {
                    return Err(Box::from(format!("no turtle named {}", name)));
                }
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
                let _ = int.ui_sender.send(UiEvent::Wait(duration.clone()));
                thread::sleep(Duration::from_millis(duration));
                Ok(Token::Void)
            },
        }
    }

    pub fn print() -> Self {
        Command {
            name: String::from("print"),
            params: vec![String::from("text")],
            action: |int: &mut Interpreter, com: &Command, args: Vec<Token>| {
                let token = decode_token(args.get(0))?;
                let text: String;
                match token {
                    Token::String(string) => text = string.clone(),
                    Token::Number(number) => text = number.to_string(),
                    Token::Boolean(boolean) => text = boolean.to_string(),
                    Token::List(list) => text = format!("[{}]", list),
                    _ => text = String::new(),
                }
                let _ = int.ui_sender.send(UiEvent::Print(text));
                Ok(Token::Void)
            },
        }
    }
}
