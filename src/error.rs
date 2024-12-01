use thiserror::Error;

use crate::lex::Token;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("{1}\n[line {num}]", num = .0.line_number)]
    ParseError(Token, String),
}
