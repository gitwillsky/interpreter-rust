use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
    environment::{Environment, Value},
    error::Error,
    interpreter::Interpreter,
    lex::Literal,
    stmt::FunctionDecl,
};
use lox_macro::New;

#[derive(Debug, Clone)]
pub enum Callable {
    Function(Function),
    NativeFunction(NativeFunction),
}

pub trait CallableInterface: ToString {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        closure_env: Rc<RefCell<Environment>>,
        arguments: Vec<Value>,
    ) -> Result<Value, Error>;
}

impl CallableInterface for Callable {
    fn arity(&self) -> usize {
        match self {
            Callable::Function(func) => func.arity(),
            Callable::NativeFunction(func) => func.arity,
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        env: Rc<RefCell<Environment>>,
        arguments: Vec<Value>,
    ) -> Result<Value, Error> {
        match self {
            Callable::Function(func) => func.call(interpreter, env, arguments),
            Callable::NativeFunction(func) => func.call(interpreter, env, arguments),
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

#[derive(Debug, New, Clone)]
pub struct Function {
    declaration: FunctionDecl,
}

impl CallableInterface for Function {
    fn arity(&self) -> usize {
        self.declaration.parameters.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        closure_env: Rc<RefCell<Environment>>,
        arguments: Vec<Value>,
    ) -> Result<Value, Error> {
        let mut env = Environment::new(Some(closure_env));
        for (param, argument) in self.declaration.parameters.iter().zip(arguments) {
            env.define(param.lexeme.clone(), argument);
        }
        let result = interpreter.execute_block(&self.declaration.body, env);
        match result {
            Ok(_) => Ok(Value::Literal(Literal::Nil)),
            Err(e) => match e {
                Error::ReturnValue(value) => Ok(value.clone()),
                _ => Err(e),
            },
        }
    }
}

impl ToString for Function {
    fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}

#[derive(Debug, New, Clone)]
pub struct NativeFunction {
    pub name: String,
    pub arity: usize,
    pub func: fn(Vec<Value>) -> Result<Value, Error>,
}

impl CallableInterface for NativeFunction {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _closure_env: Rc<RefCell<Environment>>,
        arguments: Vec<Value>,
    ) -> Result<Value, Error> {
        (self.func)(arguments)
    }
}

impl ToString for NativeFunction {
    fn to_string(&self) -> String {
        format!("<native fn {}>", self.name)
    }
}
