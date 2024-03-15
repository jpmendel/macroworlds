use crate::interpreter::event::UiEvent;
use crate::interpreter::interpreter::Interpreter;
use crate::interpreter::language::command::command::{Command, Params};
use crate::interpreter::language::token::Token;
use crate::interpreter::language::util::decode_word;
use crate::interpreter::state::object::TurtleShape;

impl Command {
    pub fn loadshape() -> Self {
        Command::reserved(
            "loadshape",
            Params::Fixed(2),
            |int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let name = decode_word(com, &args, 0)?;
                let path = decode_word(com, &args, 1)?;
                let name_ptr = name.into_boxed_str();
                if let Some(shape) = int.state.data.get_shape(&name_ptr) {
                    if let TurtleShape::Image(existing_name, existing_path) = shape {
                        // If the image is already loaded with the same name and path, ignore it without error.
                        if name_ptr == *existing_name && path == *existing_path {
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
                    TurtleShape::Image(name_ptr.clone(), path.clone()),
                );
                int.event.send_ui(UiEvent::AddShape(name_ptr, path));
                Ok(Token::Void)
            },
        )
    }

    pub fn loadtext() -> Self {
        Command::reserved(
            "loadtext",
            Params::Fixed(1),
            |_int: &mut Interpreter, com: &str, args: Vec<Token>| {
                let _path = decode_word(com, &args, 0)?;
                // TODO: Implement loading a text file.
                Ok(Token::Void)
            },
        )
    }
}
