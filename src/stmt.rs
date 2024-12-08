use lox_macro::NewFunction;

use crate::{expr::ExprEnum, lex::Token};

pub trait StmtVisitor {
    type Output;
    fn visit_expression(&mut self, stmt: &Expression) -> Self::Output;
    fn visit_print(&mut self, stmt: &Print) -> Self::Output;
    fn visit_var_decl(&mut self, stmt: &VarDecl) -> Self::Output;
    fn visit_block(&mut self, stmt: &Block) -> Self::Output;
    fn visit_if(&mut self, stmt: &If) -> Self::Output;
    fn visit_while(&mut self, stmt: &While) -> Self::Output;
    fn visit_function_decl(&mut self, stmt: &FunctionDecl) -> Self::Output;
    fn visit_return(&mut self, stmt: &Return) -> Self::Output;
}

pub trait Stmt {
    fn accept<R>(&self, visitor: &mut dyn StmtVisitor<Output = R>) -> R;
}

#[derive(Debug, Clone)]
pub enum StmtEnum {
    Expression(Expression),
    Print(Print),
    VarDecl(VarDecl),
    Block(Block),
    If(If),
    While(While),
    FunctionDecl(FunctionDecl),
    Return(Return),
}

impl Stmt for StmtEnum {
    fn accept<R>(&self, visitor: &mut dyn StmtVisitor<Output = R>) -> R {
        match self {
            Self::Expression(stmt) => visitor.visit_expression(stmt),
            Self::Print(stmt) => visitor.visit_print(stmt),
            Self::VarDecl(stmt) => visitor.visit_var_decl(stmt),
            Self::Block(stmt) => visitor.visit_block(stmt),
            Self::If(stmt) => visitor.visit_if(stmt),
            Self::While(stmt) => visitor.visit_while(stmt),
            Self::FunctionDecl(stmt) => visitor.visit_function_decl(stmt),
            Self::Return(stmt) => visitor.visit_return(stmt),
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

#[derive(NewFunction, Debug, Clone)]
pub struct While {
    pub condition: Box<ExprEnum>,
    pub body: Box<StmtEnum>,
}

#[derive(NewFunction, Debug, Clone)]
pub struct FunctionDecl {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Block,
}

#[derive(NewFunction, Debug, Clone)]
pub struct Return {
    pub keyword: Token,
    pub value: Option<Box<ExprEnum>>,
}
