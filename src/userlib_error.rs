use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Username,
    Password,
    Uid,
    Gid,
    Gecos,
    HomeDir,
    ShellDir,
}

#[derive(Debug, PartialEq)]
pub enum UserLibError {
    NotFound,
    ParseError,
    Message(String),
}

impl Display for UserLibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, ""),
            Self::ParseError => write!(f, "Failed to parse"), // TODO details
            Self::Message(message) => write!(f, "{}", message),
        }
    }
}

impl Error for UserLibError {
    fn description(&self) -> &str {
        match self {
            Self::NotFound => "not found",
            Self::ParseError => "failed to parse",
            Self::Message(message) => message,
        }
    }
}

impl From<&str> for UserLibError {
    fn from(err: &str) -> Self {
        Self::Message(err.to_owned())
    }
}
