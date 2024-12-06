use thiserror::Error;

use crate::lex::Token;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("[line {num}] [lexeme {lex}] {msg}", num = .0.line_number, lex = .0.lexeme, msg = .1)]
    ParseError(Token, String),
    #[error("{0}")]
    AssignmentError(String),
}
