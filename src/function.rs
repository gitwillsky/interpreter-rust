use crate::{interpreter::Interpreter, lex::Literal};
use anyhow::Result;

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Literal>) -> Result<Literal>;
}

#[derive(Debug, Clone)]
pub enum Function {
    Native {
        arity: usize,
        body: Box<fn(Vec<Literal>) -> Literal>,
    },
}

impl Callable for Function {
    fn arity(&self) -> usize {
        match self {
            Function::Native { arity, .. } => *arity,
        }
    }

    fn call(&self, _interpreter: &Interpreter, arguments: Vec<Literal>) -> Result<Literal> {
        match self {
            Function::Native { body, .. } => Ok(body(arguments)),
        }
    }
}

impl Function {
    pub fn new_native(arity: usize, body: Box<fn(Vec<Literal>) -> Literal>) -> Self {
        Self::Native { arity, body }
    }
}
