use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum LogtailError {
    Io(io::Error),
    Regex(regex::Error),
    InvalidInput(String),
}

impl Display for LogtailError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{err}"),
            Self::Regex(err) => write!(f, "{err}"),
            Self::InvalidInput(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for LogtailError {}

impl From<io::Error> for LogtailError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<regex::Error> for LogtailError {
    fn from(value: regex::Error) -> Self {
        Self::Regex(value)
    }
}
