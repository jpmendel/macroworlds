use crate::interpreter::event::UiEvent;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::language::command::command::{Command, Params};
use crate::interpreter::language::token::Token;
use crate::interpreter::language::util::{decode_word, query_files};
use crate::interpreter::state::object::TurtleShape;
use std::fs::{DirEntry, File};
use std::io::Read;

impl Command {
    pub fn currentdir() -> Self {
        Command::reserved(
            "currentdir",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let base = int.state.data.get_base_directory();
                Ok(Token::Word(base.clone()))
            },
        )
    }

    pub fn chdir() -> Self {
        Command::reserved(
            "chdir",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let path = decode_word(com, &args, 0)?;
                int.state.data.set_base_directory(path);
                Ok(Token::Void)
            },
        )
    }

    pub fn files() -> Self {
        Command::reserved(
            "files",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let base = int.state.data.get_base_directory();
                let files = query_files(base, |entry: &DirEntry| {
                    let Ok(meta) = entry.metadata() else {
                        return false;
                    };
                    meta.is_file()
                })?;
                Ok(Token::List(files))
            },
        )
    }

    pub fn directories() -> Self {
        Command::reserved(
            "directories",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let base = int.state.data.get_base_directory();
                let files = query_files(base, |entry: &DirEntry| {
                    let Ok(meta) = entry.metadata() else {
                        return false;
                    };
                    meta.is_dir()
                })?;
                Ok(Token::List(files))
            },
        )
    }

    pub fn pictlist() -> Self {
        Command::reserved(
            "pictlist",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let base = int.state.data.get_base_directory();
                let files = query_files(base, |entry: &DirEntry| {
                    let Ok(file_name) = entry.file_name().into_string() else {
                        return false;
                    };
                    file_name.ends_with(".jpg")
                        || file_name.ends_with(".bmp")
                        || file_name.ends_with(".png")
                })?;
                Ok(Token::List(files))
            },
        )
    }

    pub fn textlist() -> Self {
        Command::reserved(
            "textlist",
            Params::None,
            |int: &mut Interpreter, _com: &str, _args: Vec<Token>| {
                let base = int.state.data.get_base_directory();
                let files = query_files(base, |entry: &DirEntry| {
                    let Ok(file_name) = entry.file_name().into_string() else {
                        return false;
                    };
                    file_name.ends_with(".txt")
                })?;
                Ok(Token::List(files))
            },
        )
    }

    pub fn loadshape() -> Self {
        Command::reserved(
            "loadshape",
            Params::Fixed(2),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let name = decode_word(com, &args, 0)?;
                let path = decode_word(com, &args, 1)?;
                let name_ptr = name.into_boxed_str();
                let full_path = format!("{}{}", int.state.data.get_base_directory(), path);
                if let Some(shape) = int.state.data.get_shape(&name_ptr) {
                    if let TurtleShape::Image(existing_name, existing_path) = shape {
                        // If the image is already loaded with the same name and path, overwrite without error.
                        if name_ptr == *existing_name && full_path == *existing_path {
                            return Ok(Token::Void);
                        }
                    } else {
                        // If a different image has this name, throw an error to prevent overwrite.
                        return Err(Box::from(format!(
                            "shape named {} already exists",
                            name_ptr
                        )));
                    }
                }
                int.state.data.set_shape(
                    &name_ptr,
                    TurtleShape::Image(name_ptr.clone(), full_path.clone()),
                );
                int.event.send_ui(UiEvent::AddShape(name_ptr, full_path));
                Ok(Token::Void)
            },
        )
    }

    pub fn loadpict() -> Self {
        Command::reserved(
            "loadpict",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let path = decode_word(com, &args, 0)?;
                let full_path = format!("{}{}", int.state.data.get_base_directory(), path);
                int.event.send_ui(UiEvent::SetPicture(full_path));
                Ok(Token::Void)
            },
        )
    }

    pub fn loadtext() -> Self {
        Command::reserved(
            "loadtext",
            Params::Fixed(1),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let path = decode_word(com, &args, 0)?;
                let full_path = format!("{}{}", int.state.data.get_base_directory(), path);
                let mut file = match File::open(full_path) {
                    Ok(file) => file,
                    Err(err) => return Err(Box::from(format!("failed to open file: {}", err))),
                };
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                Ok(Token::Word(contents))
            },
        )
    }
}
