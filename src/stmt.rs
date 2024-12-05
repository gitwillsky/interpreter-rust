use anyhow::Result;
use lox_macro::NewFunction;

use crate::{expr::ExprEnum, lex::Token};

pub trait StmtVisitor {
    fn visit_expression(&self, stmt: &Expression) -> Result<()>;
    fn visit_print(&self, stmt: &Print) -> Result<()>;
    fn visit_var_decl(&self, stmt: &VarDecl) -> Result<()>;
    fn visit_block(&mut self, stmt: &Block) -> Result<()>;
    fn visit_if(&mut self, stmt: &If) -> Result<()>;
}

pub trait Stmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()>;
}

#[derive(Debug, Clone)]
pub enum StmtEnum {
    Expression(Expression),
    Print(Print),
    VarDecl(VarDecl),
    Block(Block),
    If(If),
}

impl Stmt for StmtEnum {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()> {
        match self {
            Self::Expression(stmt) => visitor.visit_expression(stmt),
            Self::Print(stmt) => visitor.visit_print(stmt),
            Self::VarDecl(stmt) => visitor.visit_var_decl(stmt),
            Self::Block(stmt) => visitor.visit_block(stmt),
            Self::If(stmt) => visitor.visit_if(stmt),
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

#[derive(NewFunction, Debug, Clone)]
pub struct Block {
    pub statements: Vec<StmtEnum>,
}

#[derive(NewFunction, Debug, Clone)]
pub struct If {
    pub condition: Box<ExprEnum>,
    pub then_branch: Box<StmtEnum>,
    pub else_branch: Option<Box<StmtEnum>>,
}
