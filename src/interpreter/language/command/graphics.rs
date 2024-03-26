use crate::interpreter::event::UiEvent;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::language::command::command::{Command, Params};
use crate::interpreter::language::token::Token;
use crate::interpreter::language::util::{
    decode_list, decode_number, decode_token, decode_word, join_to_list_string,
};
use crate::interpreter::state::object::{Line, Object, Point, Size, TextStyle};
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

impl Command {
    pub fn forward() -> Self {
        Command::reserved(
            "forward",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let dist = decode_number(com, &args, 0)?;
                let Object::Turtle(turtle) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                if turtle.is_locked {
                    return Ok(Token::Void);
                }
                let original_pos = turtle.pos.clone();
                let h = turtle.true_heading();
                let x = dist * h.cos();
                let y = dist * h.sin();
                let new_pos = Point::new(original_pos.x + x, original_pos.y + y);
                turtle.pos = new_pos.clone();
                int.event
                    .send_ui(UiEvent::ObjectPos(turtle.name.clone(), turtle.pos.clone()));
                if turtle.is_drawing {
                    let name = turtle.name.clone();
                    let color = turtle.color.clone();
                    let pen_size = turtle.pen_size.clone();
                    let line = Line::new(original_pos, new_pos, color, pen_size);
                    int.state.canvas.add_line(line.clone());
                    int.event.send_ui(UiEvent::AddLine(name, line));
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn back() -> Self {
        Command::reserved(
            "back",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let dist = decode_number(com, &args, 0)?;
                let Object::Turtle(turtle) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                if turtle.is_locked {
                    return Ok(Token::Void);
                }
                let original_pos = turtle.pos.clone();
                let h = turtle.true_heading();
                let x = -dist * h.cos();
                let y = -dist * h.sin();
                let new_pos = Point::new(original_pos.x + x, original_pos.y + y);
                turtle.pos = new_pos.clone();
                int.event
                    .send_ui(UiEvent::ObjectPos(turtle.name.clone(), turtle.pos.clone()));
                if turtle.is_drawing {
                    let name = turtle.name.clone();
                    let color = turtle.color.clone();
                    let pen_size = turtle.pen_size.clone();
                    let line = Line::new(original_pos, new_pos, color, pen_size);
                    int.state.canvas.add_line(line.clone());
                    int.event.send_ui(UiEvent::AddLine(name, line));
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn left() -> Self {
        Command::reserved(
            "left",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let angle = decode_number(com, &args, 0)?;
                let Object::Turtle(turtle) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                if turtle.is_locked {
                    return Ok(Token::Void);
                }
                turtle.heading -= angle;
                int.event.send_ui(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
                Ok(Token::Void)
            },
        )
    }

    pub fn right() -> Self {
        Command::reserved(
            "right",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let angle = decode_number(com, &args, 0)?;
                let Object::Turtle(turtle) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                if turtle.is_locked {
                    return Ok(Token::Void);
                }
                turtle.heading += angle;
                int.event.send_ui(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
                Ok(Token::Void)
            },
        )
    }

    pub fn xcor() -> Self {
        Command::reserved(
            "xcor",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let pos = int.state.canvas.current_object()?.pos();
                Ok(Token::Number(pos.x))
            },
        )
    }

    pub fn setx() -> Self {
        Command::reserved(
            "setx",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let x = decode_number(com, &args, 0)?;
                let object = int.state.canvas.current_object_mut()?;
                if object.is_locked() {
                    return Ok(Token::Void);
                }
                let original_pos = object.pos().clone();
                let new_pos = Point::new(x, original_pos.y);
                object.set_pos(new_pos.clone());
                int.event.send_ui(UiEvent::ObjectPos(
                    Box::from(object.name()),
                    object.pos().clone(),
                ));
                if let Object::Turtle(turtle) = object {
                    if turtle.is_drawing {
                        let name = turtle.name.clone();
                        let color = turtle.color.clone();
                        let stroke_width = turtle.pen_size.clone();
                        let line = Line::new(original_pos, new_pos, color, stroke_width);
                        int.state.canvas.add_line(line.clone());
                        int.event.send_ui(UiEvent::AddLine(name, line));
                    }
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn ycor() -> Self {
        Command::reserved(
            "ycor",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let pos = int.state.canvas.current_object()?.pos();
                Ok(Token::Number(pos.y))
            },
        )
    }

    pub fn sety() -> Self {
        Command::reserved(
            "sety",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let y = decode_number(com, &args, 0)?;
                let object = int.state.canvas.current_object_mut()?;
                if object.is_locked() {
                    return Ok(Token::Void);
                }
                let original_pos = object.pos().clone();
                let new_pos = Point::new(original_pos.x, y);
                object.set_pos(new_pos.clone());
                int.event.send_ui(UiEvent::ObjectPos(
                    Box::from(object.name()),
                    object.pos().clone(),
                ));
                if let Object::Turtle(turtle) = object {
                    if turtle.is_drawing {
                        let name = turtle.name.clone();
                        let color = turtle.color.clone();
                        let stroke_width = turtle.pen_size.clone();
                        let line = Line::new(original_pos, new_pos, color, stroke_width);
                        int.state.canvas.add_line(line.clone());
                        int.event.send_ui(UiEvent::AddLine(name, line));
                    }
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn pos() -> Self {
        Command::reserved(
            "pos",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let pos = int.state.canvas.current_object()?.pos();
                Ok(Token::List(format!("{} {}", pos.x, pos.y)))
            },
        )
    }

    pub fn setpos() -> Self {
        Command::reserved(
            "setpos",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let list = decode_list(com, &args, 0)?;
                let list_items = int.parse_list(&list, true)?;
                if list_items.len() != 2 {
                    return Err(Box::from("setpos expected 2 coordinates"));
                }
                let Some(Token::Number(x)) = list_items.get(0) else {
                    return Err(Box::from("setpos expected number for x-coordinate"));
                };
                let Some(Token::Number(y)) = list_items.get(1) else {
                    return Err(Box::from("setpos expected number for y-coordinate"));
                };
                let object = int.state.canvas.current_object_mut()?;
                if object.is_locked() {
                    return Ok(Token::Void);
                }
                let original_pos = object.pos().clone();
                let new_pos = Point::new(*x, *y);
                object.set_pos(new_pos.clone());
                int.event.send_ui(UiEvent::ObjectPos(
                    Box::from(object.name()),
                    object.pos().clone(),
                ));
                if let Object::Turtle(turtle) = object {
                    if turtle.is_drawing {
                        let name = turtle.name.clone();
                        let color = turtle.color.clone();
                        let stroke_width = turtle.pen_size.clone();
                        let line = Line::new(original_pos, new_pos, color, stroke_width);
                        int.state.canvas.add_line(line.clone());
                        int.event.send_ui(UiEvent::AddLine(name, line));
                    }
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn heading() -> Self {
        Command::reserved(
            "heading",
            Params::None,
            |int: &mut Interpreter, com: &str, _args: Vec<Token>| {
                let Object::Turtle(turtle) = int.state.canvas.current_object()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                Ok(Token::Number(turtle.heading))
            },
        )
    }

    pub fn setheading() -> Self {
        Command::reserved(
            "setheading",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let heading = decode_number(com, &args, 0)?;
                let Object::Turtle(turtle) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                if turtle.is_locked {
                    return Ok(Token::Void);
                }
                turtle.heading = heading;
                int.event.send_ui(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
                Ok(Token::Void)
            },
        )
    }

    pub fn color() -> Self {
        Command::reserved(
            "color",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let object = int.state.canvas.current_object()?;
                Ok(Token::Number(object.color()))
            },
        )
    }

    pub fn setcolor() -> Self {
        Command::reserved(
            "setcolor",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let color = decode_number(com, &args, 0)?;
                let object = int.state.canvas.current_object_mut()?;
                if object.is_locked() {
                    return Ok(Token::Void);
                }
                object.set_color(color);
                int.event.send_ui(UiEvent::ObjectColor(
                    Box::from(object.name()),
                    object.color().clone(),
                ));
                Ok(Token::Void)
            },
        )
    }

    pub fn size() -> Self {
        Command::reserved(
            "size",
            Params::None,
            |int: &mut Interpreter, com: &str, _args: Vec<Token>| {
                let Object::Turtle(turtle) = int.state.canvas.current_object()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                let size = &turtle.size;
                Ok(Token::List(format!("{} {}", size.w, size.h)))
            },
        )
    }

    pub fn setsize() -> Self {
        Command::reserved(
            "setsize",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let token = decode_token(com, &args, 0)?;
                let size = match token {
                    Token::Number(number) => Size::new(number, number),
                    Token::List(list) => {
                        let list_items = int.parse_list(&list, true)?;
                        let Some(Token::Number(width)) = list_items.get(0) else {
                            return Err(Box::from("setsize expected number for input 0"));
                        };
                        let Some(Token::Number(height)) = list_items.get(1) else {
                            return Err(Box::from("setsize expected number for input 1"));
                        };
                        Size::new(*width, *height)
                    }
                    _ => return Err(Box::from("setsize expected number or list as input")),
                };
                let Object::Turtle(turtle) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                if turtle.is_locked {
                    return Ok(Token::Void);
                }
                turtle.size = size;
                int.event.send_ui(UiEvent::ObjectSize(
                    turtle.name.clone(),
                    turtle.size.clone(),
                ));
                Ok(Token::Void)
            },
        )
    }

    pub fn pensize() -> Self {
        Command::reserved(
            "pensize",
            Params::None,
            |int: &mut Interpreter, com: &str, _args: Vec<Token>| {
                let Object::Turtle(turtle) = int.state.canvas.current_object()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                Ok(Token::Number(turtle.pen_size.clone()))
            },
        )
    }

    pub fn setpensize() -> Self {
        Command::reserved(
            "setpensize",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let size = decode_number(com, &args, 0)?;
                let Object::Turtle(turtle) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                if turtle.is_locked {
                    return Ok(Token::Void);
                }
                turtle.pen_size = size;
                Ok(Token::Void)
            },
        )
    }

    pub fn shape() -> Self {
        Command::reserved(
            "shape",
            Params::None,
            |int: &mut Interpreter, com: &str, _args: Vec<Token>| {
                let Object::Turtle(turtle) = int.state.canvas.current_object()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                Ok(Token::Word(turtle.shape.to_string()))
            },
        )
    }

    pub fn setshape() -> Self {
        Command::reserved(
            "setshape",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let shape_name = decode_word(com, &args, 0)?;
                let Some(shape) = int.state.data.get_shape(&shape_name) else {
                    return Err(Box::from(format!("no shape named {}", shape_name)));
                };
                let Object::Turtle(turtle) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                if turtle.is_locked {
                    return Ok(Token::Void);
                }
                turtle.shape = shape.clone();
                int.event.send_ui(UiEvent::TurtleShape(
                    turtle.name.clone(),
                    turtle.shape.clone(),
                ));
                Ok(Token::Void)
            },
        )
    }

    pub fn pd() -> Self {
        Command::reserved(
            "pd",
            Params::None,
            |int: &mut Interpreter, com: &str, _args: Vec<Token>| {
                let Object::Turtle(turtle) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                if turtle.is_locked {
                    return Ok(Token::Void);
                }
                turtle.is_drawing = true;
                Ok(Token::Void)
            },
        )
    }

    pub fn pu() -> Self {
        Command::reserved(
            "pu",
            Params::None,
            |int: &mut Interpreter, com: &str, _args: Vec<Token>| {
                let Object::Turtle(turtle) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                if turtle.is_locked {
                    return Ok(Token::Void);
                }
                turtle.is_drawing = false;
                Ok(Token::Void)
            },
        )
    }

    pub fn visible() -> Self {
        Command::reserved(
            "visible?",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let object = int.state.canvas.current_object()?;
                Ok(Token::Boolean(object.is_visible()))
            },
        )
    }

    pub fn st() -> Self {
        Command::reserved(
            "st",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let object = int.state.canvas.current_object_mut()?;
                if object.is_locked() {
                    return Ok(Token::Void);
                }
                object.set_visible(true);
                int.event
                    .send_ui(UiEvent::ObjectVisible(Box::from(object.name()), true));
                Ok(Token::Void)
            },
        )
    }

    pub fn ht() -> Self {
        Command::reserved(
            "ht",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let object = int.state.canvas.current_object_mut()?;
                if object.is_locked() {
                    return Ok(Token::Void);
                }
                object.set_visible(false);
                int.event
                    .send_ui(UiEvent::ObjectVisible(Box::from(object.name()), false));
                Ok(Token::Void)
            },
        )
    }

    pub fn freeze() -> Self {
        Command::reserved(
            "freeze",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let object = int.state.canvas.current_object_mut()?;
                object.set_locked(true);
                Ok(Token::Void)
            },
        )
    }

    pub fn unfreeze() -> Self {
        Command::reserved(
            "unfreeze",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let object = int.state.canvas.current_object_mut()?;
                object.set_locked(false);
                Ok(Token::Void)
            },
        )
    }

    pub fn distance() -> Self {
        Command::reserved(
            "distance",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let other_name = decode_word(com, &args, 0)?;
                let Object::Turtle(current) = int.state.canvas.current_object()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                let Object::Turtle(other) = int.state.canvas.get_object(&other_name)? else {
                    return Err(Box::from(format!(
                        "{} expected turtle name for input 0",
                        com
                    )));
                };
                let x = other.pos.x - current.pos.x;
                let y = other.pos.y - current.pos.y;
                let dist = (x.powi(2) + y.powi(2)).sqrt();
                Ok(Token::Number(dist))
            },
        )
    }

    pub fn towards() -> Self {
        Command::reserved(
            "towards",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let other_name = decode_word(com, &args, 0)?;
                let Object::Turtle(other) = int.state.canvas.get_object(&other_name)? else {
                    return Err(Box::from(format!(
                        "{} expected turtle name for input 0",
                        com
                    )));
                };
                let other_pos = other.pos.clone();
                let Object::Turtle(turtle) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                if turtle.is_locked {
                    return Ok(Token::Void);
                }
                let x = other_pos.x - turtle.pos.x;
                let y = other_pos.y - turtle.pos.y;
                let angle = y.atan2(x).to_degrees();
                turtle.heading = angle;
                int.event.send_ui(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
                Ok(Token::Void)
            },
        )
    }

    pub fn touching() -> Self {
        Command::reserved(
            "touching?",
            Params::Fixed(2),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let t1_name = decode_word(com, &args, 0)?;
                let t2_name = decode_word(com, &args, 1)?;
                let Object::Turtle(turtle1) = int.state.canvas.get_object(&t1_name)? else {
                    return Err(Box::from(format!(
                        "{} expected turtle name for input 0",
                        com
                    )));
                };
                let Object::Turtle(turtle2) = int.state.canvas.get_object(&t2_name)? else {
                    return Err(Box::from(format!(
                        "{} expected turtle name for input 1",
                        com
                    )));
                };
                if !turtle1.is_visible || !turtle2.is_visible {
                    return Ok(Token::Boolean(false));
                }
                let result = turtle1.pos.x < turtle2.pos.x + turtle2.size.w
                    && turtle1.pos.x + turtle1.size.w > turtle2.pos.x
                    && turtle1.pos.y > turtle2.pos.y + turtle2.size.h
                    && turtle1.pos.y + turtle1.size.h < turtle2.pos.y;
                Ok(Token::Boolean(result))
            },
        )
    }

    pub fn colorunder() -> Self {
        Command::reserved(
            "colorunder",
            Params::None,
            |int: &mut Interpreter, com: &str, _args: Vec<Token>| {
                let Object::Turtle(turtle) = int.state.canvas.current_object()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                let point = turtle.pos.clone();
                let color = int.state.canvas.color_at_point(&point);
                Ok(Token::Number(color))
            },
        )
    }

    pub fn fontsize() -> Self {
        Command::reserved(
            "fontsize",
            Params::None,
            |int: &mut Interpreter, com: &str, _args: Vec<Token>| {
                let Object::Text(text) = int.state.canvas.current_object()? else {
                    return Err(Box::from(format!("{} expected a text", com)));
                };
                Ok(Token::Number(text.font_size))
            },
        )
    }

    pub fn setfontsize() -> Self {
        Command::reserved(
            "setfontsize",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let font_size = decode_number(com, &args, 0)?;
                let Object::Text(text) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a text", com)));
                };
                if text.is_locked {
                    return Ok(Token::Void);
                }
                text.font_size = font_size;
                int.event
                    .send_ui(UiEvent::TextSize(text.name.clone(), text.font_size.clone()));
                Ok(Token::Void)
            },
        )
    }

    pub fn setstyle() -> Self {
        Command::reserved(
            "setstyle",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let token = decode_token(com, &args, 0)?;
                let style_set = match token {
                    Token::Word(word) => {
                        if let Some(style) = TextStyle::from(word) {
                            HashSet::from([style])
                        } else {
                            HashSet::new()
                        }
                    }
                    Token::List(list) => {
                        let list_items = int.parse_list(&list, false)?;
                        let mut style_set = HashSet::new();
                        for item in list_items {
                            let Token::Word(word) = item else {
                                continue;
                            };
                            let Some(style) = TextStyle::from(word) else {
                                continue;
                            };
                            style_set.insert(style);
                        }
                        style_set
                    }
                    _ => return Err(Box::from("setstyle expected a word or list as input")),
                };
                let Object::Text(text) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a text", com)));
                };
                if text.is_locked {
                    return Ok(Token::Void);
                }
                int.event
                    .send_ui(UiEvent::TextStyle(text.name.clone(), style_set));
                Ok(Token::Void)
            },
        )
    }

    pub fn text() -> Self {
        Command::reserved(
            "text",
            Params::None,
            |int: &mut Interpreter, com: &str, _args: Vec<Token>| {
                let Object::Text(text) = int.state.canvas.current_object()? else {
                    return Err(Box::from(format!("{} expected a text", com)));
                };
                let text_string = text.text.clone();
                Ok(Token::Word(text_string))
            },
        )
    }

    pub fn print() -> Self {
        Command::reserved(
            "print",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let token = decode_token(com, &args, 0)?;
                let string = match token {
                    Token::Word(word) => word.clone(),
                    Token::Number(number) => number.to_string(),
                    Token::List(list) => {
                        let items = int.parse_list(&list, false)?;
                        join_to_list_string(items)
                    }
                    _ => return Err(Box::from("expected word, number or list")),
                };
                let Object::Text(text) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a text", com)));
                };
                if text.is_locked {
                    return Ok(Token::Void);
                }
                text.text = string.clone();
                int.event
                    .send_ui(UiEvent::TextPrint(text.name.clone(), string));
                Ok(Token::Void)
            },
        )
    }

    pub fn cleartext() -> Self {
        Command::reserved(
            "cleartext",
            Params::None,
            |int: &mut Interpreter, com: &str, _args: Vec<Token>| {
                let Object::Text(text) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a text", com)));
                };
                if text.is_locked {
                    return Ok(Token::Void);
                }
                text.text = String::new();
                int.event.send_ui(UiEvent::TextClear(text.name.clone()));
                Ok(Token::Void)
            },
        )
    }

    pub fn projectsize() -> Self {
        Command::reserved(
            "projectsize",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let size = int.state.canvas.get_size();
                Ok(Token::List(format!("{} {}", size.w, size.h)))
            },
        )
    }

    pub fn setprojectsize() -> Self {
        Command::reserved(
            "setprojectsize",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let list = decode_list(com, &args, 0)?;
                let size: Vec<&str> = list.split(' ').collect();
                if size.len() != 2 {
                    return Err(Box::from("invalid project size"));
                }
                let width = size[0].parse::<f32>()?;
                let height = size[1].parse::<f32>()?;
                int.state.canvas.set_size(width, height);
                int.event.send_ui(UiEvent::CanvasSize(width, height));
                Ok(Token::Void)
            },
        )
    }

    pub fn bg() -> Self {
        Command::reserved(
            "bg",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let bg_color = int.state.canvas.get_bg_color();
                Ok(Token::Number(bg_color as f32))
            },
        )
    }

    pub fn setbg() -> Self {
        Command::reserved(
            "setbg",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let color = decode_number(com, &args, 0)?;
                int.state.canvas.set_bg_color(color as u8);
                int.event.send_ui(UiEvent::BgColor(color));
                Ok(Token::Void)
            },
        )
    }

    pub fn setpict() -> Self {
        Command::reserved(
            "setpict",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let pict = decode_word(com, &args, 0)?;
                if int.state.data.get_picture(&pict).is_none() {
                    return Err(Box::from(format!(
                        "placepict picture named {} does not exist",
                        pict
                    )));
                }
                int.event.send_ui(UiEvent::PlacePicture(
                    pict,
                    Point::new(
                        -int.state.canvas.get_size().w / 2.0,
                        int.state.canvas.get_size().h / 2.0,
                    ),
                    int.state.canvas.get_size().clone(),
                ));
                Ok(Token::Void)
            },
        )
    }

    pub fn placepict() -> Self {
        Command::reserved(
            "placepict",
            Params::Fixed(3),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let pict = decode_word(com, &args, 0)?;
                let pos = decode_list(com, &args, 1)?;
                let size = decode_list(com, &args, 2)?;
                if int.state.data.get_picture(&pict).is_none() {
                    return Err(Box::from(format!(
                        "placepict picture named {} does not exist",
                        pict
                    )));
                }
                let pos_items = int.parse_list(&pos, true)?;
                if pos_items.len() != 2 {
                    return Err(Box::from("placepict expected 2 coordinates in input 1"));
                }
                let size_items = int.parse_list(&size, true)?;
                if size_items.len() != 2 {
                    return Err(Box::from("placepict expected 2 dimensions in input 2"));
                }
                let Some(Token::Number(x)) = pos_items.get(0) else {
                    return Err(Box::from("placepict expected number for x-coordinate"));
                };
                let Some(Token::Number(y)) = pos_items.get(1) else {
                    return Err(Box::from("placepict expected number for y-coordinate"));
                };
                let Some(Token::Number(w)) = size_items.get(0) else {
                    return Err(Box::from("placepict expected number for width"));
                };
                let Some(Token::Number(h)) = size_items.get(1) else {
                    return Err(Box::from("placepict expected number for height"));
                };
                int.event.send_ui(UiEvent::PlacePicture(
                    pict,
                    Point::new(*x, *y),
                    Size::new(*w, *h),
                ));
                Ok(Token::Void)
            },
        )
    }

    pub fn home() -> Self {
        Command::reserved(
            "home",
            Params::None,
            |int: &mut Interpreter, com: &str, _args: Vec<Token>| {
                let Object::Turtle(turtle) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                if turtle.is_locked {
                    return Ok(Token::Void);
                }
                let original_pos = turtle.pos.clone();
                let new_pos = Point::zero();
                turtle.pos = new_pos.clone();
                turtle.heading = 0.0;
                int.event
                    .send_ui(UiEvent::ObjectPos(turtle.name.clone(), turtle.pos.clone()));
                int.event.send_ui(UiEvent::TurtleHeading(
                    turtle.name.clone(),
                    turtle.heading.clone(),
                ));
                if turtle.is_drawing {
                    let name = turtle.name.clone();
                    let color = turtle.color.clone();
                    let pen_size = turtle.pen_size.clone();
                    let line = Line::new(original_pos, new_pos, color, pen_size);
                    int.state.canvas.add_line(line.clone());
                    int.event.send_ui(UiEvent::AddLine(name, line));
                }
                Ok(Token::Void)
            },
        )
    }

    pub fn clean() -> Self {
        Command::reserved(
            "clean",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                int.state.canvas.clear();
                int.event.send_ui(UiEvent::Clean);
                Ok(Token::Void)
            },
        )
    }

    pub fn cg() -> Self {
        Command::reserved(
            "cg",
            Params::None,
            |int: &mut Interpreter, com: &str, _args: Vec<Token>| {
                int.state.canvas.clear();
                let Object::Turtle(turtle) = int.state.canvas.current_object_mut()? else {
                    return Err(Box::from(format!("{} expected a turtle", com)));
                };
                if !turtle.is_locked {
                    turtle.pos = Point::zero();
                    turtle.heading = 0.0;
                    int.event
                        .send_ui(UiEvent::ObjectPos(turtle.name.clone(), turtle.pos.clone()));
                    int.event.send_ui(UiEvent::TurtleHeading(
                        turtle.name.clone(),
                        turtle.heading.clone(),
                    ));
                }
                int.event.send_ui(UiEvent::Clean);
                Ok(Token::Void)
            },
        )
    }

    pub fn newturtle() -> Self {
        Command::reserved(
            "newturtle",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let name = decode_word(com, &args, 0)?;
                int.state.canvas.create_turtle(&name)?;
                int.event.send_ui(UiEvent::NewTurtle(name.into()));
                Ok(Token::Void)
            },
        )
    }

    pub fn newtext() -> Self {
        Command::reserved(
            "newtext",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let name = decode_word(com, &args, 0)?;
                int.state.canvas.create_text(&name)?;
                int.event.send_ui(UiEvent::NewText(name.into()));
                Ok(Token::Void)
            },
        )
    }

    pub fn remove() -> Self {
        Command::reserved(
            "remove",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let name = decode_word(com, &args, 0)?;
                int.state.canvas.remove_object(&name);
                int.event.send_ui(UiEvent::RemoveObject(name.into()));
                Ok(Token::Void)
            },
        )
    }

    pub fn wait() -> Self {
        Command::reserved(
            "wait",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let duration = decode_number(com, &args, 0)? as u64;
                int.event.send_ui(UiEvent::Wait(duration.clone()));
                thread::sleep(Duration::from_millis(duration));
                Ok(Token::Void)
            },
        )
    }

    pub fn show() -> Self {
        Command::reserved(
            "show",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let token = decode_token(com, &args, 0)?;
                let text = token.to_string();
                int.event.send_ui(UiEvent::ConsolePrint(text));
                Ok(Token::Void)
            },
        )
    }

    pub fn announce() -> Self {
        Command::reserved(
            "announce",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let token = decode_token(com, &args, 0)?;
                let text = match token {
                    Token::List(list) => list.clone(),
                    token => token.to_string(),
                };
                int.event.send_ui(UiEvent::Announce(text));
                Ok(Token::Void)
            },
        )
    }

    pub fn cc() -> Self {
        Command::reserved(
            "cc",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                int.event.send_ui(UiEvent::ClearConsole);
                Ok(Token::Void)
            },
        )
    }
}
