use std::error::Error;
use std::fmt::{self, Display};

#[allow(clippy::module_name_repetitions)]
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

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq)]
pub enum UserLibError {
    NotFound,
    ParseError,
    FilesChanged,
    FilesRequired,
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
            Self::Simple(m) => write!(f, "{}", m),
            Self::IOError(m, e) => write!(f, "{},{}", m, e),
        }
    }
}

impl Display for UserLibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "not found"),
            Self::ParseError => write!(f, "failed to parse"),
            Self::FilesRequired => write!(
                f,
                "File locking is only possible if some files are specified"
            ),
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
            Self::NotFound
            | Self::ParseError
            | Self::FilesChanged
            | Self::FilesRequired
            | Self::Message(MyMessage::Simple(_)) => None,
            Self::Message(MyMessage::IOError(_, ref e)) => Some(e),
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

impl From<std::io::Error> for UserLibError {
    fn from(e: std::io::Error) -> Self {
        Self::Message(MyMessage::Simple(e.to_string()))
    }
}

impl From<(String, std::io::Error)> for UserLibError {
    fn from((m, e): (String, std::io::Error)) -> Self {
        Self::Message(MyMessage::IOError(m, e))
    }
}
/*
impl From<(String, Error)> for UserLibError {
    fn from((m, e): (String, Error)) -> Self {
        UserLibError::Message(MyMessage::IOError(m, e))
    }
}*/
