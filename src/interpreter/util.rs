use crate::language::command::Command;
use std::error::Error;

pub fn is_return_command(command: &Command) -> bool {
    *command.name == *"output"
}

pub fn is_eof(err: &Box<dyn Error>) -> bool {
    err.to_string() == "eof"
}

pub fn is_interrupt(err: &Box<dyn Error>) -> bool {
    err.to_string() == "interrupt"
}
