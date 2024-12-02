use anyhow::Result;
use lox_macro::NewFunction;

use crate::{expr::ExprEnum, lex::Token};

pub trait StmtVisitor {
    fn visit_expression(&mut self, stmt: &Expression) -> Result<()>;
    fn visit_print(&mut self, stmt: &Print) -> Result<()>;
    fn visit_var_decl(&mut self, stmt: &VarDecl) -> Result<()>;
}

pub trait Stmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()>;
}

#[derive(Debug, Clone)]
pub enum StmtEnum {
    Expression(Expression),
    Print(Print),
    VarDecl(VarDecl),
}

impl Stmt for StmtEnum {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()> {
        match self {
            Self::Expression(stmt) => visitor.visit_expression(stmt),
            Self::Print(stmt) => visitor.visit_print(stmt),
            Self::VarDecl(stmt) => visitor.visit_var_decl(stmt),
        }
    }
}

#[derive(NewFunction, Debug, Clone)]
pub struct Expression {
    pub expression: Box<ExprEnum>,
}

#[derive(NewFunction, Debug, Clone)]
pub struct Print {
    pub expression: Box<ExprEnum>,
}

#[derive(NewFunction, Debug, Clone)]
pub struct VarDecl {
    pub name: Token,
    pub initializer: Option<Box<ExprEnum>>,
}
