use anyhow::Result;
use lox_macro::NewFunction;

use crate::expr::ExprEnum;

pub trait StmtVisitor {
    fn visit_expression(&self, stmt: &Expression) -> Result<()>;
    fn visit_print(&self, stmt: &Print) -> Result<()>;
}

pub trait Stmt {
    fn accept(&self, visitor: &dyn StmtVisitor) -> Result<()>;
}

#[derive(Debug)]
pub enum StmtEnum {
    Expression(Expression),
    Print(Print),
}

impl Stmt for StmtEnum {
    fn accept(&self, visitor: &dyn StmtVisitor) -> Result<()> {
        match self {
            Self::Expression(stmt) => visitor.visit_expression(stmt),
            Self::Print(stmt) => visitor.visit_print(stmt),
        }
    }
}

#[derive(NewFunction, Debug)]
pub struct Expression {
    pub expression: Box<ExprEnum>,
}

#[derive(NewFunction, Debug)]
pub struct Print {
    pub expression: Box<ExprEnum>,
}
