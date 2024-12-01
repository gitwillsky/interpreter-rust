use crate::{
    expr::{Binary, Expr, ExprEnum, ExprVisitor, Grouping, Literal as ExprLiteral, Unary},
    lex::{Literal, TokenType},
};
use anyhow::{bail, Result};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    fn evaluate(&self, expr: &ExprEnum) -> Result<Literal> {
        expr.accept(self)
    }

    pub fn interpret(&self, expr: &ExprEnum) -> Result<String> {
        let value = self.evaluate(expr)?;
        match value {
            Literal::Nil => Ok("nil".to_string()),
            Literal::String(s) => Ok(s),
            Literal::Number(n) => Ok(n.to_string()),
            Literal::Boolean(b) => Ok(b.to_string()),
        }
    }
}

impl ExprVisitor for Interpreter {
    type Output = Result<Literal>;

    fn visit_binary(&self, expr: &Binary) -> Self::Output {
        let right = self.evaluate(expr.right.as_ref())?;
        let left = self.evaluate(expr.left.as_ref())?;

        match expr.operator.token_type {
            TokenType::Plus => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Number(left + right))
                }
                (Literal::String(left), Literal::String(right)) => {
                    Ok(Literal::String(left + &right))
                }
                _ => bail!("Operand must be two numbers or two strings."),
            },
            TokenType::Minus => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Number(left - right))
                }
                _ => bail!("Operand must be a numbers."),
            },
            TokenType::Slash => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Number(left / right))
                }
                _ => bail!("Operand must be a number."),
            },
            TokenType::Star => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Number(left * right))
                }
                _ => bail!("Operand must be a number."),
            },
            TokenType::Greater => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Boolean(left > right))
                }
                _ => bail!("Operand must be numbers."),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Boolean(left >= right))
                }
                _ => bail!("Operand must be numbers."),
            },
            TokenType::Less => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Boolean(left < right))
                }
                _ => bail!("Operand must be numbers."),
            },
            TokenType::LessEqual => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Boolean(left <= right))
                }
                _ => bail!("Operand must be numbers."),
            },
            TokenType::EqualEqual => Ok(Literal::Boolean(left.is_equal(&right))),
            TokenType::BangEqual => Ok(Literal::Boolean(!left.is_equal(&right))),
            _ => todo!(),
        }
    }

    fn visit_grouping(&self, expr: &Grouping) -> Self::Output {
        self.evaluate(expr.expression.as_ref())
    }

    fn visit_literal(&self, expr: &ExprLiteral) -> Self::Output {
        Ok(expr.value.clone())
    }

    fn visit_unary(&self, expr: &Unary) -> Self::Output {
        let right = self.evaluate(expr.right.as_ref())?;

        match expr.operator.token_type {
            TokenType::Minus => match right {
                Literal::Number(d) => Ok(Literal::Number(-d)),
                _ => bail!("Operand must be a number."),
            },
            TokenType::Bang => Ok(Literal::Boolean(!right.is_truthy())),
            _ => bail!("Unknown operator."),
        }
    }
}
