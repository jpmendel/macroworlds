use crate::interpreter::event::UiEvent;
use crate::interpreter::interpreter::Interpreter;
use crate::language::command::{Command, Params};
use crate::language::token::Token;
use crate::language::util::{
    decode_list, decode_number, decode_token, decode_word, join_to_list_string,
};
use crate::state::object::{CanvasObject, Line, TurtleShape};
use std::f32::consts::PI;
use std::thread;
use std::time::Duration;

impl Command {
    pub fn forward() -> Self {
        Command::reserved(
            String::from("forward"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let dist = decode_number(com, &args, 0)?;
                let turtle = int.state.canvas.current_turtle()?;
                let original_pos = turtle.pos.clone();
                let h = turtle.true_heading();
                let x = dist * h.cos();
                let y = dist * h.sin();
                let new_pos = (original_pos.0 + x, original_pos.1 + y);
                turtle.pos = new_pos.clone();
                int.event
                    .send_ui_event(UiEvent::ObjectPos(turtle.name.clone(), turtle.pos.clone()));
                if turtle.is_drawing {
                    let name = turtle.name.clone();
                    let color = turtle.color.clone();
                    let pen_size = turtle.pen_size.clone();
                    let line = Line::from(original_pos, new_pos, color, pen_size);
                    int.state.canvas.add_line(line.clone());
                    int.event.send_ui_event(UiEvent::AddLine(name, line));
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn back() -> Self {
        Command::reserved(
            String::from("back"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let dist = decode_number(com, &args, 0)?;
                let turtle = int.state.canvas.current_turtle()?;
                let original_pos = turtle.pos.clone();
                let h = turtle.true_heading();
                let x = -dist * h.cos();
                let y = -dist * h.sin();
                let new_pos = (original_pos.0 + x, original_pos.1 + y);
                turtle.pos = new_pos.clone();
                int.event
                    .send_ui_event(UiEvent::ObjectPos(turtle.name.clone(), turtle.pos.clone()));
                if turtle.is_drawing {
                    let name = turtle.name.clone();
                    let color = turtle.color.clone();
                    let pen_size = turtle.pen_size.clone();
                    let line = Line::from(original_pos, new_pos, color, pen_size);
                    int.state.canvas.add_line(line.clone());
                    int.event.send_ui_event(UiEvent::AddLine(name, line));
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn left() -> Self {
        Command::reserved(
            String::from("left"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let angle = decode_number(com, &args, 0)?;
                let turtle = int.state.canvas.current_turtle()?;
                turtle.heading -= angle;
                int.event.send_ui_event(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
                Ok(Token::Void)
            },
        )
    }

    pub fn right() -> Self {
        Command::reserved(
            String::from("right"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let angle = decode_number(com, &args, 0)?;
                let turtle = int.state.canvas.current_turtle()?;
                turtle.heading += angle;
                int.event.send_ui_event(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
                Ok(Token::Void)
            },
        )
    }

    pub fn xcor() -> Self {
        Command::reserved(
            String::from("xcor"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let pos = int.state.canvas.current_object()?.pos();
                Ok(Token::Number(pos.0))
            },
        )
    }

    pub fn ycor() -> Self {
        Command::reserved(
            String::from("ycor"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let pos = int.state.canvas.current_object()?.pos();
                Ok(Token::Number(pos.1))
            },
        )
    }

    pub fn pos() -> Self {
        Command::reserved(
            String::from("pos"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let pos = int.state.canvas.current_object()?.pos();
                Ok(Token::List(format!("{} {}", pos.0, pos.1)))
            },
        )
    }

    pub fn heading() -> Self {
        Command::reserved(
            String::from("heading"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.canvas.current_turtle()?;
                Ok(Token::Number(turtle.heading))
            },
        )
    }

    pub fn color() -> Self {
        Command::reserved(
            String::from("color"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let obj = int.state.canvas.current_object()?;
                Ok(Token::Number(obj.color().clone()))
            },
        )
    }

    pub fn pensize() -> Self {
        Command::reserved(
            String::from("pensize"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.canvas.current_turtle()?;
                Ok(Token::Number(turtle.pen_size.clone()))
            },
        )
    }

    pub fn shape() -> Self {
        Command::reserved(
            String::from("shape"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.canvas.current_turtle()?;
                Ok(Token::Word(turtle.shape.to_string()))
            },
        )
    }

    pub fn setx() -> Self {
        Command::reserved(
            String::from("setx"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let x = decode_number(com, &args, 0)?;
                let obj = int.state.canvas.current_object()?;
                let original_pos = obj.pos().clone();
                let new_pos = (x, original_pos.1);
                obj.set_pos(new_pos.clone());
                int.event
                    .send_ui_event(UiEvent::ObjectPos(obj.name().clone(), obj.pos().clone()));
                if let CanvasObject::Turtle(turtle) = obj {
                    if turtle.is_drawing {
                        let name = turtle.name.clone();
                        let color = turtle.color.clone();
                        let pen_size = turtle.pen_size.clone();
                        let line = Line::from(original_pos, new_pos, color, pen_size);
                        int.state.canvas.add_line(line.clone());
                        int.event.send_ui_event(UiEvent::AddLine(name, line));
                    }
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn sety() -> Self {
        Command::reserved(
            String::from("sety"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let y = decode_number(com, &args, 0)?;
                let obj = int.state.canvas.current_object()?;
                let original_pos = obj.pos().clone();
                let new_pos = (original_pos.0, y);
                obj.set_pos(new_pos.clone());
                int.event
                    .send_ui_event(UiEvent::ObjectPos(obj.name().clone(), obj.pos().clone()));
                if let CanvasObject::Turtle(turtle) = obj {
                    if turtle.is_drawing {
                        let name = turtle.name.clone();
                        let color = turtle.color.clone();
                        let pen_size = turtle.pen_size.clone();
                        let line = Line::from(original_pos, new_pos, color, pen_size);
                        int.state.canvas.add_line(line.clone());
                        int.event.send_ui_event(UiEvent::AddLine(name, line));
                    }
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn setpos() -> Self {
        Command::reserved(
            String::from("setpos"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let list = decode_list(com, &args, 0)?;
                let coords: Vec<&str> = list.split(' ').collect();
                if coords.len() != 2 {
                    return Err(Box::from("invalid coordinates"));
                }
                let x = coords[0].parse::<f32>()?;
                let y = coords[1].parse::<f32>()?;
                let obj = int.state.canvas.current_object()?;
                let original_pos = obj.pos().clone();
                let new_pos = (x, y);
                obj.set_pos(new_pos.clone());
                int.event
                    .send_ui_event(UiEvent::ObjectPos(obj.name().clone(), obj.pos().clone()));
                if let CanvasObject::Turtle(turtle) = obj {
                    if turtle.is_drawing {
                        let name = turtle.name.clone();
                        let color = turtle.color.clone();
                        let pen_size = turtle.pen_size.clone();
                        let line = Line::from(original_pos, new_pos, color, pen_size);
                        int.state.canvas.add_line(line.clone());
                        int.event.send_ui_event(UiEvent::AddLine(name, line));
                    }
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn seth() -> Self {
        Command::reserved(
            String::from("seth"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let heading = decode_number(com, &args, 0)?;
                let turtle = int.state.canvas.current_turtle()?;
                turtle.heading = heading;
                int.event.send_ui_event(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
                Ok(Token::Void)
            },
        )
    }

    pub fn setc() -> Self {
        Command::reserved(
            String::from("setc"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let color = decode_number(com, &args, 0)?;
                let obj = int.state.canvas.current_object()?;
                obj.set_color(color);
                int.event.send_ui_event(UiEvent::ObjectColor(
                    obj.name().clone(),
                    obj.color().clone(),
                ));
                Ok(Token::Void)
            },
        )
    }

    pub fn setpensize() -> Self {
        Command::reserved(
            String::from("setpensize"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let size = decode_number(com, &args, 0)?;
                let turtle = int.state.canvas.current_turtle()?;
                turtle.pen_size = size;
                Ok(Token::Void)
            },
        )
    }

    pub fn setsh() -> Self {
        Command::reserved(
            String::from("setsh"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let shape_string = decode_word(com, &args, 0)?;
                let shape = match shape_string.to_lowercase().as_str() {
                    "triangle" => TurtleShape::Triangle,
                    "circle" => TurtleShape::Circle,
                    "square" => TurtleShape::Square,
                    sh => return Err(Box::from(format!("no shape named {}", sh))),
                };
                let turtle = int.state.canvas.current_turtle()?;
                turtle.shape = shape;
                int.event.send_ui_event(UiEvent::TurtleShape(
                    turtle.name.clone(),
                    turtle.shape.clone(),
                ));
                Ok(Token::Void)
            },
        )
    }

    pub fn pd() -> Self {
        Command::reserved(
            String::from("pd"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.canvas.current_turtle()?;
                turtle.is_drawing = true;
                Ok(Token::Void)
            },
        )
    }

    pub fn pu() -> Self {
        Command::reserved(
            String::from("pu"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.canvas.current_turtle()?;
                turtle.is_drawing = false;
                Ok(Token::Void)
            },
        )
    }

    pub fn st() -> Self {
        Command::reserved(
            String::from("st"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.canvas.current_turtle()?;
                turtle.is_visible = true;
                int.event
                    .send_ui_event(UiEvent::ObjectVisible(turtle.name.clone(), true));
                Ok(Token::Void)
            },
        )
    }

    pub fn ht() -> Self {
        Command::reserved(
            String::from("ht"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.canvas.current_turtle()?;
                turtle.is_visible = false;
                int.event
                    .send_ui_event(UiEvent::ObjectVisible(turtle.name.clone(), false));
                Ok(Token::Void)
            },
        )
    }

    pub fn distance() -> Self {
        Command::reserved(
            String::from("distance"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let other = decode_word(com, &args, 0)?;
                let current_pos = int.state.canvas.current_turtle()?.pos.clone();
                let other_pos = int.state.canvas.get_turtle(&other)?.pos.clone();
                let x = other_pos.0 - current_pos.0;
                let y = other_pos.1 - current_pos.1;
                let dist = (x.powi(2) + y.powi(2)).sqrt();
                Ok(Token::Number(dist))
            },
        )
    }

    pub fn towards() -> Self {
        Command::reserved(
            String::from("towards"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let other = decode_word(com, &args, 0)?;
                let current_pos = int.state.canvas.current_turtle()?.pos.clone();
                let other_pos = int.state.canvas.get_turtle(&other)?.pos.clone();
                let x = other_pos.0 - current_pos.0;
                let y = other_pos.1 - current_pos.1;
                let angle = y.atan2(x) * 180.0 / PI;
                let turtle = int.state.canvas.current_turtle()?;
                turtle.heading = angle;
                int.event.send_ui_event(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
                Ok(Token::Void)
            },
        )
    }

    pub fn colorunder() -> Self {
        Command::reserved(
            String::from("colorunder"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.canvas.current_turtle()?;
                let point = turtle.pos.clone();
                let color = int.state.canvas.color_at_point(&point);
                Ok(Token::Number(color))
            },
        )
    }

    pub fn setfontsize() -> Self {
        Command::reserved(
            String::from("setfontsize"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let font_size = decode_number(com, &args, 0)?;
                let text = int.state.canvas.current_text()?;
                text.font_size = font_size;
                int.event
                    .send_ui_event(UiEvent::TextSize(text.name.clone(), text.font_size.clone()));
                Ok(Token::Void)
            },
        )
    }

    pub fn showtext() -> Self {
        Command::reserved(
            String::from("showtext"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let text = int.state.canvas.current_text()?;
                text.is_visible = true;
                int.event
                    .send_ui_event(UiEvent::ObjectVisible(text.name.clone(), true));
                Ok(Token::Void)
            },
        )
    }

    pub fn hidetext() -> Self {
        Command::reserved(
            String::from("hidetext"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let text = int.state.canvas.current_text()?;
                text.is_visible = false;
                int.event
                    .send_ui_event(UiEvent::ObjectVisible(text.name.clone(), false));
                Ok(Token::Void)
            },
        )
    }

    pub fn print() -> Self {
        Command::reserved(
            String::from("print"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let token = decode_token(com, &args, 0)?;
                let string = match token {
                    Token::Word(word) => word.clone(),
                    Token::Number(number) => number.to_string(),
                    Token::List(list) => {
                        let items = int.parse_list(&list)?;
                        join_to_list_string(items)
                    }
                    _ => return Err(Box::from("expected word, number or list")),
                };
                let text = int.state.canvas.current_text()?;
                text.text = string.clone();
                int.event
                    .send_ui_event(UiEvent::TextPrint(text.name.clone(), string));
                Ok(Token::Void)
            },
        )
    }

    pub fn cleartext() -> Self {
        Command::reserved(
            String::from("cleartext"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let text = int.state.canvas.current_text()?;
                text.text = String::new();
                int.event
                    .send_ui_event(UiEvent::TextClear(text.name.clone()));
                Ok(Token::Void)
            },
        )
    }

    pub fn newprojectsize() -> Self {
        Command::reserved(
            String::from("newprojectsize"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let list = decode_list(com, &args, 0)?;
                let size: Vec<&str> = list.split(' ').collect();
                if size.len() != 2 {
                    return Err(Box::from("invalid size"));
                }
                let width = size[0].parse::<f32>()?;
                let height = size[1].parse::<f32>()?;
                int.state.canvas.set_size(width, height);
                int.event.send_ui_event(UiEvent::CanvasSize(width, height));
                Ok(Token::Void)
            },
        )
    }

    pub fn bg() -> Self {
        Command::reserved(
            String::from("bg"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let bg_color = int.state.canvas.get_bg_color();
                Ok(Token::Number(bg_color as f32))
            },
        )
    }

    pub fn setbg() -> Self {
        Command::reserved(
            String::from("setbg"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let color = decode_number(com, &args, 0)?;
                int.state.canvas.set_bg_color(color as u8);
                int.event.send_ui_event(UiEvent::BgColor(color));
                Ok(Token::Void)
            },
        )
    }

    pub fn home() -> Self {
        Command::reserved(
            String::from("home"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                let turtle = int.state.canvas.current_turtle()?;
                let original_pos = turtle.pos.clone();
                let new_pos = (0.0, 0.0);
                turtle.pos = new_pos.clone();
                turtle.heading = 0.0;
                int.event
                    .send_ui_event(UiEvent::ObjectPos(turtle.name.clone(), turtle.pos.clone()));
                int.event.send_ui_event(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
                if turtle.is_drawing {
                    let name = turtle.name.clone();
                    let color = turtle.color.clone();
                    let pen_size = turtle.pen_size.clone();
                    let line = Line::from(original_pos, new_pos, color, pen_size);
                    int.state.canvas.add_line(line.clone());
                    int.event.send_ui_event(UiEvent::AddLine(name, line));
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn clean() -> Self {
        Command::reserved(
            String::from("clean"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                int.state.canvas.clear();
                int.event.send_ui_event(UiEvent::Clean);
                Ok(Token::Void)
            },
        )
    }

    pub fn cg() -> Self {
        Command::reserved(
            String::from("cg"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                int.state.canvas.clear();
                let turtle = int.state.canvas.current_turtle()?;
                turtle.pos = (0.0, 0.0);
                turtle.heading = 0.0;
                int.event
                    .send_ui_event(UiEvent::ObjectPos(turtle.name.clone(), turtle.pos.clone()));
                int.event.send_ui_event(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
                int.event.send_ui_event(UiEvent::Clean);
                Ok(Token::Void)
            },
        )
    }

    pub fn newturtle() -> Self {
        Command::reserved(
            String::from("newturtle"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let name = decode_word(com, &args, 0)?;
                int.state.canvas.create_turtle(name.clone())?;
                int.event.send_ui_event(UiEvent::NewTurtle(name));
                Ok(Token::Void)
            },
        )
    }

    pub fn newtext() -> Self {
        Command::reserved(
            String::from("newtext"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let name = decode_word(com, &args, 0)?;
                int.state.canvas.create_text(name.clone())?;
                int.event.send_ui_event(UiEvent::NewText(name));
                Ok(Token::Void)
            },
        )
    }

    pub fn remove() -> Self {
        Command::reserved(
            String::from("remove"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let name = decode_word(com, &args, 0)?;
                int.state.canvas.remove_object(&name);
                int.event.send_ui_event(UiEvent::RemoveObject(name));
                Ok(Token::Void)
            },
        )
    }

    pub fn wait() -> Self {
        Command::reserved(
            String::from("wait"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let duration = decode_number(com, &args, 0)? as u64;
                int.event.send_ui_event(UiEvent::Wait(duration.clone()));
                thread::sleep(Duration::from_millis(duration));
                Ok(Token::Void)
            },
        )
    }

    pub fn show() -> Self {
        Command::reserved(
            String::from("show"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let token = decode_token(com, &args, 0)?;
                let text: String;
                match token {
                    Token::Word(string) => text = string.clone(),
                    Token::Number(number) => text = number.to_string(),
                    Token::Boolean(boolean) => text = boolean.to_string(),
                    Token::List(list) => text = format!("[{}]", list),
                    _ => text = String::new(),
                }
                int.event.send_ui_event(UiEvent::ConsolePrint(text));
                Ok(Token::Void)
            },
        )
    }

    pub fn announce() -> Self {
        Command::reserved(
            String::from("announce"),
            Params::Fixed(1),
            |int: &mut Interpreter, com: &String, args: Vec<Token>| {
                let token = decode_token(com, &args, 0)?;
                let text: String;
                match token {
                    Token::Word(string) => text = string.clone(),
                    Token::Number(number) => text = number.to_string(),
                    Token::Boolean(boolean) => text = boolean.to_string(),
                    Token::List(list) => text = list.clone(),
                    _ => text = String::new(),
                }
                int.event.send_ui_event(UiEvent::Announce(text));
                Ok(Token::Void)
            },
        )
    }

    pub fn cc() -> Self {
        Command::reserved(
            String::from("cc"),
            Params::None,
            |int: &mut Interpreter, _com: &String, _args: Vec<Token>| {
                int.event.send_ui_event(UiEvent::ClearConsole);
                Ok(Token::Void)
            },
        )
    }
}
