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
    FilesChanged,
    Message(String),
}

impl Display for UserLibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "not found"),
            Self::ParseError => write!(f, "failed to parse"),
            Self::FilesChanged => write!(
                f,
                "The files changed. Updating could lead to conflict aborting."
            ),
            Self::Message(message) => write!(f, "{}", message),
        }
    }
}

impl Error for UserLibError {
    fn description(&self) -> &str {
        todo!()
    }
}

impl From<&str> for UserLibError {
    fn from(err: &str) -> Self {
        Self::Message(err.to_owned())
    }
}

impl From<String> for UserLibError {
    fn from(err: String) -> Self {
        Self::Message(err)
    }
}
