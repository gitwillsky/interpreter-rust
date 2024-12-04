use anyhow::{bail, Ok, Result};
use std::{cell::RefCell, collections::HashMap};

use crate::lex::Literal;

pub struct Environment<'a> {
    enclosing: Option<&'a RefCell<Environment<'a>>>,
    values: HashMap<String, Literal>,
}

impl<'a> Environment<'a> {
    pub fn new(enclosing: Option<&'a RefCell<Environment<'a>>>) -> Self {
        Self {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        // 在定义前并没有查找是否已经存在，即允许重复定义变量
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Literal> {
        self.values.get(name).cloned().or_else(|| {
            self.enclosing
                .as_ref()
                .and_then(|enclosing| enclosing.borrow().get(name))
        })
    }

    pub fn assign(&mut self, name: String, value: Literal) -> Result<()> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            Ok(())
        } else {
            match self.enclosing.as_mut() {
                Some(parent) => parent.borrow_mut().assign(name, value),
                None => bail!("Undefined variable {name}"),
            }
        }
    }
}
