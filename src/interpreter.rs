use crate::{
    expr::{Binary, Expr, ExprEnum, ExprVisitor, Grouping, Literal as ExprLiteral, Unary},
    lex::{Literal, TokenType},
};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    fn evaluate(&self, expr: &ExprEnum) -> Literal {
        expr.accept(self)
    }

    pub fn interpret(&self, expr: &ExprEnum) {
        let value = self.evaluate(expr);
        match value {
            Literal::Nil => println!("nil"),
            Literal::String(s) => println!("{s}"),
            Literal::Number(n) => println!("{n}"),
            Literal::Boolean(b) => println!("{b}"),
        }
    }
}

impl ExprVisitor for Interpreter {
    type Output = Literal;

    fn visit_binary(&self, expr: &Binary) -> Self::Output {
        let right = self.evaluate(expr.right.as_ref());
        let left = self.evaluate(expr.left.as_ref());

        match expr.operator.token_type {
            TokenType::Plus => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => Literal::Number(left + right),
                (Literal::String(left), Literal::String(right)) => Literal::String(left + &right),
                _ => todo!(),
            },
            TokenType::Minus => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => Literal::Number(left - right),
                _ => todo!(),
            },
            TokenType::Slash => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => Literal::Number(left / right),
                _ => todo!(),
            },
            TokenType::Star => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => Literal::Number(left * right),
                _ => todo!(),
            },
            TokenType::Greater => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => Literal::Boolean(left > right),
                _ => todo!(),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => Literal::Boolean(left >= right),
                _ => todo!(),
            },
            TokenType::Less => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => Literal::Boolean(left < right),
                _ => todo!(),
            },
            TokenType::LessEqual => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => Literal::Boolean(left <= right),
                _ => todo!(),
            },
            TokenType::Equal => Literal::Boolean(left.is_equal(&right)),
            _ => todo!(),
        }
    }

    fn visit_grouping(&self, expr: &Grouping) -> Self::Output {
        self.evaluate(expr.expression.as_ref())
    }

    fn visit_literal(&self, expr: &ExprLiteral) -> Self::Output {
        expr.value.clone()
    }

    fn visit_unary(&self, expr: &Unary) -> Self::Output {
        let right = self.evaluate(expr.right.as_ref());

        match expr.operator.token_type {
            TokenType::Minus => match right {
                Literal::Number(d) => Literal::Number(-d),
                _ => todo!(),
            },
            TokenType::Bang => Literal::Boolean(!right.is_truthy()),
            _ => todo!(),
        }
    }
}
