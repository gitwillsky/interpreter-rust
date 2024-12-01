use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Parse error: {0}")]
    ParseError(String),
}
