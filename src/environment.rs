use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::Error, function::Callable, lex::Literal};

#[derive(Debug, Clone)]
pub enum Value {
    Literal(Literal),
    Callable(Callable, Rc<RefCell<Environment>>),
}

impl Value {
    pub fn as_literal(&self) -> Result<Literal, Error> {
        match self {
            Self::Literal(literal) => Ok(literal.clone()),
            _ => Err(Error::RuntimeError("Value is not a literal".to_string())),
        }
    }

    pub fn as_callable(&self) -> Result<(Callable, Rc<RefCell<Environment>>), Error> {
        match self {
            Self::Callable(callable, env) => Ok((callable.clone(), env.clone())),
            _ => Err(Error::RuntimeError("Value is not a callable".to_string())),
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Self::Literal(literal) => format!("{}", literal),
            Self::Callable(callable, _) => callable.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        // 在定义前并没有查找是否已经存在，即允许重复定义变量
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.values.get(name).cloned().or_else(|| {
            self.enclosing
                .as_ref()
                .and_then(|enclosing| enclosing.borrow().get(name))
        })
    }

    pub fn get_at(&self, distance: usize, name: &str) -> Option<Value> {
        if distance == 0 {
            return self.values.get(name).cloned();
        }
        self.enclosing
            .as_ref()
            .and_then(|enclosing| enclosing.borrow().get_at(distance - 1, name))
    }

    pub fn assign_at(&mut self, distance: usize, name: String, value: Value) -> Result<(), Error> {
        if distance == 0 {
            self.values.insert(name, value);
            Ok(())
        } else {
            self.enclosing
                .as_ref()
                .unwrap()
                .borrow_mut()
                .assign_at(distance - 1, name, value)
        }
    }

    pub fn assign(&mut self, name: String, value: Value) -> Result<(), Error> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            Ok(())
        } else {
            match self.enclosing {
                Some(ref parent) => parent.borrow_mut().assign(name, value),
                None => Err(Error::RuntimeError(format!("Undefined variable {name}"))),
            }
        }
    }
}
