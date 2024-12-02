use anyhow::{bail, Ok, Result};
use std::collections::HashMap;

use crate::lex::Literal;

pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        // 在定义前并没有查找是否已经存在，即允许重复定义变量
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Literal> {
        self.values.get(name)
    }

    pub fn assign(&mut self, name: String, value: Literal) -> Result<()> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            Ok(())
        } else {
            bail!("Undefined variable {name}.")
        }
    }
}
