use std::error::Error;

pub fn eof_error() -> Box<dyn Error> {
    Box::from("eof")
}

pub fn interrupt_error() -> Box<dyn Error> {
    Box::from("interrupt")
}

pub fn is_eof(err: &Box<dyn Error>) -> bool {
    err.to_string() == "eof"
}

pub fn is_interrupt(err: &Box<dyn Error>) -> bool {
    err.to_string() == "interrupt"
}
