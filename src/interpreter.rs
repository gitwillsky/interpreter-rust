use std::cell::RefCell;

use crate::{
    environment::Environment,
    error::RuntimeError,
    expr::{
        Assignment, Binary, Expr, ExprEnum, ExprVisitor, Grouping, Literal as ExprLiteral, Unary,
        Variable,
    },
    lex::{Literal, TokenType},
    stmt::{Block, Expression, Print, Stmt, StmtEnum, StmtVisitor, VarDecl},
};
use anyhow::{bail, Result};

pub struct Interpreter<'a> {
    environment: RefCell<Environment<'a>>,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        Self {
            environment: RefCell::new(Environment::new(None)),
        }
    }

    pub fn interpret(&'a self, statements: &[StmtEnum]) -> Result<()> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn evaluate(&self, expr: &ExprEnum) -> Result<Literal> {
        expr.accept(self)
    }

    fn execute(&'a self, stmt: &dyn Stmt) -> Result<()> {
        stmt.accept(self)
    }

    fn execute_block(&'a self, statements: &[StmtEnum], new_env: Environment<'a>) -> Result<()> {
        let old_env = self.environment.replace(new_env);
        let r = statements.iter().try_for_each(|s| self.execute(s));
        self.environment.replace(old_env);
        r
    }
}

impl<'a> ExprVisitor for Interpreter<'a> {
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

    fn visit_variable(&self, expr: &Variable) -> Self::Output {
        let value = self.environment.borrow().get(&expr.name.lexeme);
        match value {
            Some(v) => Ok(v.clone()),
            None => bail!(RuntimeError::ParseError(
                expr.name.clone(),
                format!("Undefined variable '{}'", expr.name.lexeme)
            )),
        }
    }

    fn visit_assignment(&self, expr: &Assignment) -> Self::Output {
        let name = &expr.name;
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow_mut()
            .assign(name.lexeme.clone(), value.clone())
            .map_err(|e| RuntimeError::ParseError(name.clone(), e.to_string()))?;
        Ok(value)
    }
}

impl<'a> StmtVisitor<'a> for Interpreter<'a> {
    fn visit_expression(&self, stmt: &Expression) -> Result<()> {
        self.evaluate(stmt.expression.as_ref())?;
        Ok(())
    }

    fn visit_print(&self, stmt: &Print) -> Result<()> {
        let value = self.evaluate(stmt.expression.as_ref())?;
        println!("{value}");
        Ok(())
    }

    fn visit_var_decl(&self, stmt: &VarDecl) -> Result<()> {
        let value = stmt
            .initializer
            .as_ref()
            .map(|expr| self.evaluate(expr))
            .transpose()?;

        match value {
            Some(value) => self
                .environment
                .borrow_mut()
                .define(stmt.name.lexeme.clone(), value),
            None =>
            // 允许定义一个未初始化的变量
            {
                self.environment
                    .borrow_mut()
                    .define(stmt.name.lexeme.clone(), Literal::Nil)
            }
        }
        Ok(())
    }

    fn visit_block(&'a self, stmt: &Block) -> Result<()> {
        let new_env = {
            let env = Environment::new(Some(&self.environment));
            env
        };
        self.execute_block(&stmt.statements, new_env)
    }
}
