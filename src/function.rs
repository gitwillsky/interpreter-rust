use std::fmt::Debug;

use crate::{
    environment::{Environment, Value},
    error::ReturnValue,
    interpreter::Interpreter,
    lex::Literal,
    stmt::FunctionDecl,
};
use anyhow::Result;
use lox_macro::NewFunction;

#[derive(Debug, Clone)]
pub enum Callable {
    Function(Function),
    NativeFunction(NativeFunction),
}

pub trait CallableInterface: ToString {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value>;
}

impl CallableInterface for Callable {
    fn arity(&self) -> usize {
        match self {
            Callable::Function(func) => func.arity(),
            Callable::NativeFunction(func) => func.arity,
        }
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
        match self {
            Callable::Function(func) => func.call(interpreter, arguments),
            Callable::NativeFunction(func) => func.call(interpreter, arguments),
        }
    }
}

impl ToString for Callable {
    fn to_string(&self) -> String {
        match self {
            Callable::Function(func) => func.to_string(),
            Callable::NativeFunction(func) => func.to_string(),
        }
    }
}

#[derive(Debug, NewFunction, Clone)]
pub struct Function {
    declaration: FunctionDecl,
}

impl CallableInterface for Function {
    fn arity(&self) -> usize {
        self.declaration.parameters.len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
        let mut env = Environment::new(Some(interpreter.environment.clone()));
        for (param, argument) in self.declaration.parameters.iter().zip(arguments) {
            env.define(param.lexeme.clone(), argument);
        }
        let result = interpreter.execute_block(&self.declaration.body, env);
        match result {
            Ok(_) => Ok(Value::Literal(Literal::Nil)),
            Err(e) => match e.downcast_ref::<ReturnValue>() {
                Some(value) => Ok(value.0.clone()),
                None => Err(e),
            },
        }
    }
}

impl ToString for Function {
    fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}

#[derive(Debug, NewFunction, Clone)]
pub struct NativeFunction {
    pub name: String,
    pub arity: usize,
    pub func: fn(Vec<Value>) -> Result<Value>,
}

impl CallableInterface for NativeFunction {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(&self, _interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value> {
        (self.func)(arguments)
    }
}

impl ToString for NativeFunction {
    fn to_string(&self) -> String {
        format!("<native fn {}>", self.name)
    }
}
