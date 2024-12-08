use std::{
    error::Error as StdError,
    fmt::{self, Display},
};

use crate::{environment::Value, lex::Token};

#[derive(Debug)]
pub enum Error {
    InternalError(String),
    ParseError(Token, String),
    AssignmentError(String),
    RuntimeError(String),
    ReturnValue(Value),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InternalError(msg) => write!(f, "{}", msg),
            Self::ParseError(token, msg) => write!(
                f,
                "[line {}] [lexeme {}] {}",
                token.line_number, token.lexeme, msg
            ),
            Self::AssignmentError(msg) => write!(f, "{}", msg),
            Self::RuntimeError(msg) => write!(f, "{}", msg),
            Self::ReturnValue(value) => write!(f, "{}", value.to_string()),
        }
    }
}

impl StdError for Error {}
