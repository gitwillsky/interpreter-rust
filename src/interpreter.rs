use crate::{
    error::RuntimeError,
    expr::{Binary, Expr, ExprEnum, ExprVisitor, Grouping, Literal as ExprLiteral, Unary},
    lex::{Literal, TokenType},
    stmt::{Expression, Print, Stmt, StmtEnum, StmtVisitor},
};
use anyhow::{bail, Result};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&self, statements: &[StmtEnum]) -> Result<()> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn evaluate(&self, expr: &ExprEnum) -> Result<Literal> {
        expr.accept(self)
    }

    fn execute(&self, stmt: &dyn Stmt) -> Result<()> {
        stmt.accept(self)
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
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be two numbers or two strings.".into(),
                )),
            },
            TokenType::Minus => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Number(left - right))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be a numbers.".into(),
                )),
            },
            TokenType::Slash => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Number(left / right))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be a number.".into(),
                )),
            },
            TokenType::Star => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Number(left * right))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be a number.".into(),
                )),
            },
            TokenType::Greater => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Boolean(left > right))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be numbers.".into(),
                )),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Boolean(left >= right))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be numbers.".into(),
                )),
            },
            TokenType::Less => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Boolean(left < right))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be numbers.".into(),
                )),
            },
            TokenType::LessEqual => match (left, right) {
                (Literal::Number(left), Literal::Number(right)) => {
                    Ok(Literal::Boolean(left <= right))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be numbers.".into(),
                )),
            },
            TokenType::EqualEqual => Ok(Literal::Boolean(left.is_equal(&right))),
            TokenType::BangEqual => Ok(Literal::Boolean(!left.is_equal(&right))),
            _ => bail!(RuntimeError::ParseError(
                expr.operator.clone(),
                "Unknown operator.".into(),
            )),
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
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be a number.".into(),
                )),
            },
            TokenType::Bang => Ok(Literal::Boolean(!right.is_truthy())),
            _ => bail!(RuntimeError::ParseError(
                expr.operator.clone(),
                "Unknown unary operator.".into(),
            )),
        }
    }
}

impl StmtVisitor for Interpreter {
    fn visit_expression(&self, stmt: &Expression) -> Result<()> {
        self.evaluate(stmt.expression.as_ref())?;
        Ok(())
    }

    fn visit_print(&self, stmt: &Print) -> Result<()> {
        let value = self.evaluate(stmt.expression.as_ref())?;
        println!("{value:?}");
        Ok(())
    }
}
