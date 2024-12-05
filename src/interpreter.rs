use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    error::RuntimeError,
    expr::{
        Assignment, Binary, Expr, ExprEnum, ExprVisitor, Grouping, Literal as ExprLiteral, Unary,
        Variable,
    },
    lex::{Literal, TokenType},
    stmt::{Block, Expression, If, Print, Stmt, StmtEnum, StmtVisitor, VarDecl},
};
use anyhow::{bail, Result};

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new(None))),
        }
    }

    pub fn interpret(&mut self, statements: &[StmtEnum]) -> Result<()> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn evaluate(&self, expr: &ExprEnum) -> Result<Literal> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &dyn Stmt) -> Result<()> {
        stmt.accept(self)
    }

    fn execute_block(&mut self, statements: &[StmtEnum], new_env: Environment) -> Result<()> {
        let old_env = self.environment.clone();
        self.environment = Rc::new(RefCell::new(new_env));
        let r = statements.iter().try_for_each(|s| {
            let r = s.accept(self);
            r
        });
        self.environment = old_env;
        r
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

impl StmtVisitor for Interpreter {
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

    fn visit_block(&mut self, stmt: &Block) -> Result<()> {
        let new_env = {
            let env = Environment::new(Some(self.environment.clone()));
            env
        };
        self.execute_block(&stmt.statements, new_env)
    }

    fn visit_if(&mut self, stmt: &If) -> Result<()> {
        let condition = self.evaluate(stmt.condition.as_ref())?;
        if condition.is_truthy() {
            self.execute(stmt.then_branch.as_ref())?;
        } else if let Some(else_branch) = stmt.else_branch.as_ref() {
            self.execute(else_branch.as_ref())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{lex::Tokenizer, parser::Parser};

    use super::*;

    #[test]
    fn test_nested_blocks() {
        let source = r#"
var quz = "global quz";
var foo = "global foo";
var baz = "global baz";
{
  var quz = "outer quz";
  var foo = "outer foo";
  {
    var quz = "inner quz";
    print quz;
    print foo;
    print baz;
  }
  print quz;
  print foo;
  print baz;
}
print quz;
print foo;
print baz;
        "#;

        let mut tokenizer = Tokenizer::new(source.to_string());
        let (tokens, exit_code) = tokenizer.parse();
        assert_eq!(exit_code, 0);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        assert!(statements.is_ok());
        let mut interpreter = Interpreter::new();
        let r = interpreter.interpret(&statements.unwrap());
        assert!(r.is_ok());
    }
}
