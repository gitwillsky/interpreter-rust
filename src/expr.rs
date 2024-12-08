use std::fmt::Debug;

use lox_macro::NewFunction;

use crate::lex::{Literal as LiteralValue, Token};

pub trait ExprVisitor {
    type Output;
    fn visit_binary(&mut self, expr: &Binary) -> Self::Output;
    fn visit_grouping(&mut self, expr: &Grouping) -> Self::Output;
    fn visit_literal(&mut self, expr: &Literal) -> Self::Output;
    fn visit_unary(&mut self, expr: &Unary) -> Self::Output;
    fn visit_variable(&mut self, expr: &Variable) -> Self::Output;
    fn visit_assignment(&mut self, expr: &Assignment) -> Self::Output;
    fn visit_logical(&mut self, expr: &Logical) -> Self::Output;
    fn visit_call(&mut self, expr: &Call) -> Self::Output;
}
pub trait Expr: Debug {
    fn accept<R>(&self, visitor: &mut dyn ExprVisitor<Output = R>) -> R;
}

#[derive(Debug, Clone)]
pub enum ExprEnum {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable),
    Assignment(Assignment),
    Logical(Logical),
    Call(Call),
}

impl Expr for ExprEnum {
    fn accept<R>(&self, visitor: &mut dyn ExprVisitor<Output = R>) -> R {
        match self {
            ExprEnum::Binary(expr) => visitor.visit_binary(expr),
            ExprEnum::Grouping(expr) => visitor.visit_grouping(expr),
            ExprEnum::Literal(expr) => visitor.visit_literal(expr),
            ExprEnum::Unary(expr) => visitor.visit_unary(expr),
            ExprEnum::Variable(expr) => visitor.visit_variable(expr),
            ExprEnum::Assignment(expr) => visitor.visit_assignment(expr),
            ExprEnum::Logical(expr) => visitor.visit_logical(expr),
            ExprEnum::Call(expr) => visitor.visit_call(expr),
        }
    }
}

#[derive(NewFunction, Debug, Clone)]
pub struct Assignment {
    pub name: Token,
    pub value: Box<ExprEnum>,
}

#[derive(NewFunction, Debug, Clone)]
pub struct Binary {
    pub left: Box<ExprEnum>,
    pub operator: Token,
    pub right: Box<ExprEnum>,
}

#[derive(NewFunction, Debug, Clone)]
pub struct Grouping {
    pub expression: Box<ExprEnum>,
}

#[derive(NewFunction, Debug, Clone)]
pub struct Literal {
    pub value: LiteralValue,
}

#[derive(NewFunction, Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<ExprEnum>,
}

#[derive(NewFunction, Debug, Clone)]
pub struct Variable {
    pub name: Token,
}

#[derive(NewFunction, Debug, Clone)]
pub struct Logical {
    pub left: Box<ExprEnum>,
    pub operator: Token,
    pub right: Box<ExprEnum>,
}

#[derive(NewFunction, Debug, Clone)]
pub struct Call {
    pub callee: Box<ExprEnum>,
    pub paren: Token, // 保存右括号标记，用于错误信息展示
    pub arguments: Vec<ExprEnum>,
}
