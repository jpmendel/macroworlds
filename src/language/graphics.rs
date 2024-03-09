use crate::interpreter::event::UiEvent;
use crate::interpreter::interpreter::Interpreter;
use crate::language::command::{Command, Params};
use crate::language::token::Token;
use crate::language::util::{
    decode_list, decode_number, decode_token, decode_word, join_to_list_string,
};
use crate::state::object::CanvasObject;
use std::thread;
use std::time::Duration;

impl Command {
    pub fn fd() -> Self {
        Command {
            name: String::from("fd"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let dist = decode_number(args.get(0))?;
                let turtle = int.state.current_turtle()?;
                let original_pos = turtle.pos.clone();
                // Translate heading to a "clockwise, 0 == north" system.
                let x = dist * (-turtle.heading + 90.0).to_radians().cos();
                let y = dist * (-turtle.heading + 90.0).to_radians().sin();
                let new_pos = (original_pos.0 + x, original_pos.1 + y);
                turtle.pos = new_pos.clone();
                let _ = int
                    .ui_sender
                    .send(UiEvent::ObjectPos(turtle.name.clone(), turtle.pos.clone()));
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
                let turtle = int.state.current_turtle()?;
                let original_pos = turtle.pos.clone();
                // Translate heading to a "clockwise, 0 == north" system.
                let x = -dist * (-turtle.heading + 90.0).to_radians().cos();
                let y = -dist * (-turtle.heading + 90.0).to_radians().sin();
                let new_pos = (original_pos.0 + x, original_pos.1 + y);
                turtle.pos = new_pos.clone();
                let _ = int
                    .ui_sender
                    .send(UiEvent::ObjectPos(turtle.name.clone(), turtle.pos.clone()));
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
                let turtle = int.state.current_turtle()?;
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
                let turtle = int.state.current_turtle()?;
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
                let pos = int.state.current_object()?.pos();
                Ok(Token::Number(pos.0))
            },
        }
    }

    pub fn ycor() -> Self {
        Command {
            name: String::from("ycor"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let pos = int.state.current_object()?.pos();
                Ok(Token::Number(pos.1))
            },
        }
    }

    pub fn pos() -> Self {
        Command {
            name: String::from("pos"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let pos = int.state.current_object()?.pos();
                Ok(Token::List(format!("{} {}", pos.0, pos.1)))
            },
        }
    }

    pub fn heading() -> Self {
        Command {
            name: String::from("heading"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.current_turtle()?;
                Ok(Token::Number(turtle.heading))
            },
        }
    }

    pub fn color() -> Self {
        Command {
            name: String::from("color"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let color = int.state.current_object()?.color();
                Ok(Token::Number(color.clone()))
            },
        }
    }

    pub fn setx() -> Self {
        Command {
            name: String::from("setx"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let x = decode_number(args.get(0))?;
                let obj = int.state.current_object()?;
                let original_pos = obj.pos().clone();
                let new_pos = (x, original_pos.1);
                obj.set_pos(new_pos.clone());
                let _ = int
                    .ui_sender
                    .send(UiEvent::ObjectPos(obj.name().clone(), obj.pos().clone()));
                if let CanvasObject::Turtle(turtle) = obj {
                    if turtle.is_drawing {
                        let color = turtle.color.clone();
                        let line = int.state.add_line(original_pos, new_pos, color);
                        let _ = int.ui_sender.send(UiEvent::AddLine(line.clone()));
                    }
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
                let obj = int.state.current_object()?;
                let original_pos = obj.pos().clone();
                let new_pos = (original_pos.0, y);
                obj.set_pos(new_pos.clone());
                let _ = int
                    .ui_sender
                    .send(UiEvent::ObjectPos(obj.name().clone(), obj.pos().clone()));
                if let CanvasObject::Turtle(turtle) = obj {
                    if turtle.is_drawing {
                        let color = turtle.color.clone();
                        let line = int.state.add_line(original_pos, new_pos, color);
                        let _ = int.ui_sender.send(UiEvent::AddLine(line.clone()));
                    }
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
                let obj = int.state.current_object()?;
                let original_pos = obj.pos().clone();
                let new_pos = (x, y);
                obj.set_pos(new_pos.clone());
                let _ = int
                    .ui_sender
                    .send(UiEvent::ObjectPos(obj.name().clone(), obj.pos().clone()));
                if let CanvasObject::Turtle(turtle) = obj {
                    if turtle.is_drawing {
                        let color = turtle.color.clone();
                        let line = int.state.add_line(original_pos, new_pos, color);
                        let _ = int.ui_sender.send(UiEvent::AddLine(line.clone()));
                    }
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
                let turtle = int.state.current_turtle()?;
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
                let turtle = int.state.current_turtle()?;
                turtle.color = color;
                let _ = int.ui_sender.send(UiEvent::ObjectColor(
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
                let turtle = int.state.current_turtle()?;
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
                let turtle = int.state.current_turtle()?;
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
                let turtle = int.state.current_turtle()?;
                turtle.is_visible = true;
                let _ = int
                    .ui_sender
                    .send(UiEvent::ObjectVisible(turtle.name.clone(), true));
                Ok(Token::Void)
            },
        }
    }

    pub fn ht() -> Self {
        Command {
            name: String::from("ht"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.current_turtle()?;
                turtle.is_visible = false;
                let _ = int
                    .ui_sender
                    .send(UiEvent::ObjectVisible(turtle.name.clone(), false));
                Ok(Token::Void)
            },
        }
    }

    pub fn settc() -> Self {
        Command {
            name: String::from("settc"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let color = decode_number(args.get(0))?;
                let text = int.state.current_text()?;
                text.color = color;
                let _ = int
                    .ui_sender
                    .send(UiEvent::ObjectColor(text.name.clone(), text.color.clone()));
                Ok(Token::Void)
            },
        }
    }

    pub fn setfontsize() -> Self {
        Command {
            name: String::from("setfontsize"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let font_size = decode_number(args.get(0))?;
                let text = int.state.current_text()?;
                text.font_size = font_size;
                let _ = int
                    .ui_sender
                    .send(UiEvent::TextSize(text.name.clone(), text.font_size.clone()));
                Ok(Token::Void)
            },
        }
    }

    pub fn showtext() -> Self {
        Command {
            name: String::from("showtext"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let text = int.state.current_text()?;
                text.is_visible = true;
                let _ = int
                    .ui_sender
                    .send(UiEvent::ObjectVisible(text.name.clone(), true));
                Ok(Token::Void)
            },
        }
    }

    pub fn hidetext() -> Self {
        Command {
            name: String::from("hidetext"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let text = int.state.current_text()?;
                text.is_visible = false;
                let _ = int
                    .ui_sender
                    .send(UiEvent::ObjectVisible(text.name.clone(), false));
                Ok(Token::Void)
            },
        }
    }

    pub fn print() -> Self {
        Command {
            name: String::from("print"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let token = decode_token(args.get(0))?;
                let string = match token {
                    Token::Word(word) => word.clone(),
                    Token::Number(number) => number.to_string(),
                    Token::List(list) => {
                        let items = int.parse_list(list)?;
                        join_to_list_string(items)
                    }
                    _ => return Err(Box::from("expected word, number or list")),
                };
                let text = int.state.current_text()?;
                text.text = string.clone();
                let _ = int
                    .ui_sender
                    .send(UiEvent::TextText(text.name.clone(), string));
                Ok(Token::Void)
            },
        }
    }

    pub fn ct() -> Self {
        Command {
            name: String::from("ct"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let text = int.state.current_text()?;
                text.text = String::new();
                let _ = int
                    .ui_sender
                    .send(UiEvent::TextText(text.name.clone(), String::new()));
                Ok(Token::Void)
            },
        }
    }

    pub fn home() -> Self {
        Command {
            name: String::from("home"),
            params: Params::None,
            action: |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.current_turtle()?;
                let original_pos = turtle.pos.clone();
                let new_pos = (0.0, 0.0);
                turtle.pos = new_pos.clone();
                turtle.heading = 0.0;
                let _ = int
                    .ui_sender
                    .send(UiEvent::ObjectPos(turtle.name.clone(), turtle.pos.clone()));
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
                let turtle = int.state.current_turtle()?;
                turtle.pos = (0.0, 0.0);
                turtle.heading = 0.0;
                let _ = int
                    .ui_sender
                    .send(UiEvent::ObjectPos(turtle.name.clone(), turtle.pos.clone()));
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

    pub fn newtext() -> Self {
        Command {
            name: String::from("newtext"),
            params: Params::Fixed(1),
            action: |int: &mut Interpreter, _com: &String, args: Vec<Token>| {
                let name = decode_word(args.get(0))?;
                int.state.create_text(name.clone());
                let _ = int.ui_sender.send(UiEvent::NewText(name));
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
                int.state.remove_object(&name);
                let _ = int.ui_sender.send(UiEvent::RemoveObject(name));
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
