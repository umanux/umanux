use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum UserLibError {
    NotFound,
    Message(String),
}

impl Display for UserLibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, ""),
            Self::Message(message) => write!(f, "{}", message),
        }
    }
}

impl Error for UserLibError {
    fn description(&self) -> &str {
        match self {
            Self::NotFound => "not found",
            Self::Message(message) => message,
        }
    }
}

impl From<String> for UserLibError {
    fn from(err: String) -> Self {
        Self::Message(err)
    }
}
