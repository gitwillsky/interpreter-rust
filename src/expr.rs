use std::fmt::Debug;

use lox_macro::NewFunction;

use crate::lex::{Literal as LiteralValue, Token};

pub trait ExprVisitor {
    type Output;
    fn visit_binary(&self, expr: &Binary) -> Self::Output;
    fn visit_grouping(&self, expr: &Grouping) -> Self::Output;
    fn visit_literal(&self, expr: &Literal) -> Self::Output;
    fn visit_unary(&self, expr: &Unary) -> Self::Output;
}
pub trait Expr: Debug {
    fn accept<R>(&self, visitor: &dyn ExprVisitor<Output = R>) -> R;
}

#[derive(Debug)]
pub enum ExprEnum {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

impl Expr for ExprEnum {
    fn accept<R>(&self, visitor: &dyn ExprVisitor<Output = R>) -> R {
        match self {
            ExprEnum::Binary(expr) => visitor.visit_binary(expr),
            ExprEnum::Grouping(expr) => visitor.visit_grouping(expr),
            ExprEnum::Literal(expr) => visitor.visit_literal(expr),
            ExprEnum::Unary(expr) => visitor.visit_unary(expr),
        }
    }
}

#[derive(NewFunction, Debug)]
pub struct Binary {
    pub left: Box<ExprEnum>,
    pub operator: Token,
    pub right: Box<ExprEnum>,
}

#[derive(NewFunction, Debug)]
pub struct Grouping {
    pub expression: Box<ExprEnum>,
}

#[derive(NewFunction, Debug)]
pub struct Literal {
    pub value: LiteralValue,
}

#[derive(NewFunction, Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<ExprEnum>,
}
