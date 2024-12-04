use std::fmt::Debug;

use lox_macro::NewFunction;

use crate::lex::{Literal as LiteralValue, Token};

pub trait ExprVisitor {
    type Output;
    fn visit_binary(&self, expr: &Binary) -> Self::Output;
    fn visit_grouping(&self, expr: &Grouping) -> Self::Output;
    fn visit_literal(&self, expr: &Literal) -> Self::Output;
    fn visit_unary(&self, expr: &Unary) -> Self::Output;
    fn visit_variable(&self, expr: &Variable) -> Self::Output;
    fn visit_assignment(&self, expr: &Assignment) -> Self::Output;
}
pub trait Expr: Debug {
    fn accept<R>(&self, visitor: &dyn ExprVisitor<Output = R>) -> R;
}

#[derive(Debug, Clone)]
pub enum ExprEnum {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable),
    Assignment(Assignment),
}

impl Expr for ExprEnum {
    fn accept<R>(&self, visitor: &dyn ExprVisitor<Output = R>) -> R {
        match self {
            ExprEnum::Binary(expr) => visitor.visit_binary(expr),
            ExprEnum::Grouping(expr) => visitor.visit_grouping(expr),
            ExprEnum::Literal(expr) => visitor.visit_literal(expr),
            ExprEnum::Unary(expr) => visitor.visit_unary(expr),
            ExprEnum::Variable(expr) => visitor.visit_variable(expr),
            ExprEnum::Assignment(expr) => visitor.visit_assignment(expr),
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
