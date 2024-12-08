use anyhow::{bail, Result};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{function::Callable, lex::Literal};

#[derive(Debug, Clone)]
pub enum Value {
    Literal(Literal),
    Callable(Callable),
}

impl Value {
    pub fn as_literal(&self) -> Result<Literal> {
        match self {
            Self::Literal(literal) => Ok(literal.clone()),
            _ => bail!("Value is not a literal"),
        }
    }

    pub fn as_callable(&self) -> Result<Callable> {
        match self {
            Self::Callable(callable) => Ok(callable.clone()),
            _ => bail!("Value is not a callable"),
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Self::Literal(literal) => format!("{}", literal),
            Self::Callable(callable) => callable.to_string(),
        }
    }
}

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

    pub fn assign(&mut self, name: String, value: Value) -> Result<()> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            Ok(())
        } else {
            match self.enclosing {
                Some(ref parent) => parent.borrow_mut().assign(name, value),
                None => bail!("Undefined variable {name}"),
            }
        }
    }
}
