use std::{error, fmt};

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt (&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "config error {}", self.message)
    }
}

impl error::Error for Error {}
