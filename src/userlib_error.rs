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
    Message(MyMessage),
}

#[derive(Debug)]
pub enum MyMessage {
    Simple(String),
    IOError(String, std::io::Error),
}

impl PartialEq for MyMessage {
    fn eq(&self, other: &Self) -> bool {
        format!("{}", self).eq(&format!("{}", other))
    }
}

impl Display for MyMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyMessage::Simple(m) => write!(f, "{}", m),
            MyMessage::IOError(m, e) => write!(f, "{},{}", m, e),
        }
    }
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
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            UserLibError::NotFound | UserLibError::ParseError | UserLibError::FilesChanged => None,
            UserLibError::Message(MyMessage::IOError(_, ref e)) => Some(e),
            UserLibError::Message(MyMessage::Simple(_)) => None,
        }
    }
}

impl From<&str> for UserLibError {
    fn from(err: &str) -> Self {
        Self::Message(MyMessage::Simple(err.to_owned()))
    }
}

impl From<String> for UserLibError {
    fn from(err: String) -> Self {
        Self::Message(MyMessage::Simple(err))
    }
}

impl From<(String, std::io::Error)> for UserLibError {
    fn from((m, e): (String, std::io::Error)) -> Self {
        UserLibError::Message(MyMessage::IOError(m, e))
    }
}
/*
impl From<(String, Error)> for UserLibError {
    fn from((m, e): (String, Error)) -> Self {
        UserLibError::Message(MyMessage::IOError(m, e))
    }
}*/
