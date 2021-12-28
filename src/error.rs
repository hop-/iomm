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
