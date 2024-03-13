use std::error::Error;

pub fn is_eof(err: &Box<dyn Error>) -> bool {
    err.to_string() == "eof"
}

pub fn is_interrupt(err: &Box<dyn Error>) -> bool {
    err.to_string() == "interrupt"
}
