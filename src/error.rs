use std::{
    error::Error,
    fmt::{self, Display},
};
use thiserror::Error as ThisError;

use crate::{environment::Value, lex::Token};

#[derive(Debug, ThisError)]
pub enum RuntimeError {
    #[error("[line {num}] [lexeme {lex}] {msg}", num = .0.line_number, lex = .0.lexeme, msg = .1)]
    ParseError(Token, String),
    #[error("{0}")]
    AssignmentError(String),
}

#[derive(Debug)]
pub struct ReturnValue(pub Value);

impl Display for ReturnValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl Error for ReturnValue {}
