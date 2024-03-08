use crate::interpreter::event::UiEvent;
use crate::interpreter::interpreter::Interpreter;
use crate::language::command::{Command, Params};
use crate::language::token::Token;
use crate::language::util::{decode_list, decode_number, decode_token, decode_word};
use std::thread;
use std::time::Duration;

impl Command {
    pub fn fd() -> Self {
        Command {
            name: String::from("fd"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let dist = decode_number(args.get(0))?;
                let turtle = int.state.current_turtle();
                let original_pos = turtle.pos.clone();
                // Translate heading to a "clockwise, 0 == north" system.
                let x = dist * (-turtle.heading + 90.0).to_radians().cos();
                let y = dist * (-turtle.heading + 90.0).to_radians().sin();
                let new_pos = (original_pos.0 + x, original_pos.1 + y);
                turtle.pos = new_pos.clone();
                let _ = int
                    .ui_sender
                    .send(UiEvent::TurtlePos(turtle.name.clone(), turtle.pos.clone()));
                if turtle.is_drawing {
                    let color = turtle.color.clone();
                    let line = int.state.add_line(original_pos, new_pos, color);
                    let _ = int.ui_sender.send(UiEvent::AddLine(line.clone()));
                }
                Ok(Token::Void)
            },
        }
    }

    pub fn bk() -> Self {
        Command {
            name: String::from("bk"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let dist = decode_number(args.get(0))?;
                let turtle = int.state.current_turtle();
                let original_pos = turtle.pos.clone();
                // Translate heading to a "clockwise, 0 == north" system.
                let x = -dist * (-turtle.heading + 90.0).to_radians().cos();
                let y = -dist * (-turtle.heading + 90.0).to_radians().sin();
                let new_pos = (original_pos.0 + x, original_pos.1 + y);
                turtle.pos = new_pos.clone();
                let _ = int
                    .ui_sender
                    .send(UiEvent::TurtlePos(turtle.name.clone(), turtle.pos.clone()));
                if turtle.is_drawing {
                    let color = turtle.color.clone();
                    let line = int.state.add_line(original_pos, new_pos, color);
                    let _ = int.ui_sender.send(UiEvent::AddLine(line.clone()));
                }
                Ok(Token::Void)
            },
        }
    }

    pub fn lt() -> Self {
        Command {
            name: String::from("lt"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let angle = decode_number(args.get(0))?;
                let turtle = int.state.current_turtle();
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
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let angle = decode_number(args.get(0))?;
                let turtle = int.state.current_turtle();
                turtle.heading += angle;
                let _ = int.ui_sender.send(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
                Ok(Token::Void)
            },
        }
    }

    pub fn xcor() -> Self {
        Command {
            name: String::from("xcor"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.current_turtle();
                Ok(Token::Number(turtle.pos.0))
            },
        }
    }

    pub fn ycor() -> Self {
        Command {
            name: String::from("ycor"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.current_turtle();
                Ok(Token::Number(turtle.pos.1))
            },
        }
    }

    pub fn pos() -> Self {
        Command {
            name: String::from("pos"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.current_turtle();
                Ok(Token::List(format!("{} {}", turtle.pos.0, turtle.pos.1)))
            },
        }
    }

    pub fn heading() -> Self {
        Command {
            name: String::from("heading"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.current_turtle();
                Ok(Token::Number(turtle.heading))
            },
        }
    }

    pub fn color() -> Self {
        Command {
            name: String::from("color"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.current_turtle();
                Ok(Token::Number(turtle.color))
            },
        }
    }

    pub fn setx() -> Self {
        Command {
            name: String::from("setx"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let x = decode_number(args.get(0))?;
                let turtle = int.state.current_turtle();
                let original_pos = turtle.pos.clone();
                turtle.pos.0 = x;
                let new_pos = turtle.pos.clone();
                let _ = int
                    .ui_sender
                    .send(UiEvent::TurtlePos(turtle.name.clone(), turtle.pos.clone()));
                if turtle.is_drawing {
                    let color = turtle.color.clone();
                    let line = int.state.add_line(original_pos, new_pos, color);
                    let _ = int.ui_sender.send(UiEvent::AddLine(line.clone()));
                }
                Ok(Token::Void)
            },
        }
    }

    pub fn sety() -> Self {
        Command {
            name: String::from("sety"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let y = decode_number(args.get(0))?;
                let turtle = int.state.current_turtle();
                let original_pos = turtle.pos.clone();
                turtle.pos.1 = y;
                let new_pos = turtle.pos.clone();
                let _ = int
                    .ui_sender
                    .send(UiEvent::TurtlePos(turtle.name.clone(), turtle.pos.clone()));
                if turtle.is_drawing {
                    let color = turtle.color.clone();
                    let line = int.state.add_line(original_pos, new_pos, color);
                    let _ = int.ui_sender.send(UiEvent::AddLine(line.clone()));
                }
                Ok(Token::Void)
            },
        }
    }

    pub fn setpos() -> Self {
        Command {
            name: String::from("setpos"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let list = decode_list(args.get(0))?;
                let coords: Vec<&str> = list.split(' ').collect();
                if coords.len() != 2 {
                    return Err(Box::from("invalid coordinates"));
                }
                let x = coords[0].parse::<f32>()?;
                let y = coords[1].parse::<f32>()?;
                let turtle = int.state.current_turtle();
                let original_pos = turtle.pos.clone();
                let new_pos = (x, y);
                turtle.pos = new_pos.clone();
                let _ = int
                    .ui_sender
                    .send(UiEvent::TurtlePos(turtle.name.clone(), turtle.pos.clone()));
                if turtle.is_drawing {
                    let color = turtle.color.clone();
                    let line = int.state.add_line(original_pos, new_pos, color);
                    let _ = int.ui_sender.send(UiEvent::AddLine(line.clone()));
                }
                Ok(Token::Void)
            },
        }
    }

    pub fn seth() -> Self {
        Command {
            name: String::from("seth"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let heading = decode_number(args.get(0))?;
                let turtle = int.state.current_turtle();
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
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let color = decode_number(args.get(0))?;
                let turtle = int.state.current_turtle();
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
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.current_turtle();
                turtle.is_drawing = true;
                Ok(Token::Void)
            },
        }
    }

    pub fn pu() -> Self {
        Command {
            name: String::from("pu"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.current_turtle();
                turtle.is_drawing = false;
                Ok(Token::Void)
            },
        }
    }

    pub fn st() -> Self {
        Command {
            name: String::from("st"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.current_turtle();
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
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.current_turtle();
                turtle.is_visible = false;
                let _ = int
                    .ui_sender
                    .send(UiEvent::TurtleVisible(turtle.name.clone(), false));
                Ok(Token::Void)
            },
        }
    }

    pub fn home() -> Self {
        Command {
            name: String::from("home"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.current_turtle();
                let original_pos = turtle.pos.clone();
                let new_pos = (0.0, 0.0);
                turtle.pos = new_pos.clone();
                turtle.heading = 0.0;
                let _ = int
                    .ui_sender
                    .send(UiEvent::TurtlePos(turtle.name.clone(), turtle.pos.clone()));
                let _ = int.ui_sender.send(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
                if turtle.is_drawing {
                    let color = turtle.color.clone();
                    let line = int.state.add_line(original_pos, new_pos, color);
                    let _ = int.ui_sender.send(UiEvent::AddLine(line.clone()));
                }
                Ok(Token::Void)
            },
        }
    }

    pub fn clean() -> Self {
        Command {
            name: String::from("clean"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let _ = int.ui_sender.send(UiEvent::Clean);
                Ok(Token::Void)
            },
        }
    }

    pub fn cg() -> Self {
        Command {
            name: String::from("cg"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.current_turtle();
                turtle.pos = (0.0, 0.0);
                turtle.heading = 0.0;
                let _ = int
                    .ui_sender
                    .send(UiEvent::TurtlePos(turtle.name.clone(), turtle.pos.clone()));
                let _ = int.ui_sender.send(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
                let _ = int.ui_sender.send(UiEvent::Clean);
                Ok(Token::Void)
            },
        }
    }

    pub fn newturtle() -> Self {
        Command {
            name: String::from("newturtle"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let name = decode_word(args.get(0))?;
                int.state.create_turtle(name.clone());
                let _ = int.ui_sender.send(UiEvent::NewTurtle(name));
                Ok(Token::Void)
            },
        }
    }

    pub fn remove() -> Self {
        Command {
            name: String::from("remove"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let name = decode_word(args.get(0))?;
                int.state.remove_turtle(&name);
                let _ = int.ui_sender.send(UiEvent::RemoveTurtle(name));
                Ok(Token::Void)
            },
        }
    }

    pub fn wait() -> Self {
        Command {
            name: String::from("wait"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let duration = decode_number(args.get(0))? as u64;
                let _ = int.ui_sender.send(UiEvent::Wait(duration.clone()));
                thread::sleep(Duration::from_millis(duration));
                Ok(Token::Void)
            },
        }
    }

    pub fn show() -> Self {
        Command {
            name: String::from("show"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let token = decode_token(args.get(0))?;
                let text: String;
                match token {
                    Token::Word(string) => text = string.clone(),
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

    pub fn cc() -> Self {
        Command {
            name: String::from("cc"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let _ = int.ui_sender.send(UiEvent::Print(String::new()));
                Ok(Token::Void)
            },
        }
    }
}
