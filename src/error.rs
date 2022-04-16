use std::{error, fmt};

#[derive(Debug)]
pub struct Error {
}

impl fmt::Display for Error {
    fn fmt (&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "error happend")
    }
}

impl error::Error for Error {}

#[derive(Debug)]
pub struct TextError {
    pub message: String,
}

impl TextError {
    pub fn new(message: &str) -> TextError {
        TextError { message: message.to_string() }
    }
}

impl fmt::Display for TextError {
    fn fmt (&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "error {}", self.message)
    }
}

impl error::Error for TextError {}
