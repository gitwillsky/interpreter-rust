use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::{Environment, Value},
    error::{ReturnValue, RuntimeError},
    expr::{
        Assignment, Binary, Call, Expr, ExprEnum, ExprVisitor, Grouping, Literal as ExprLiteral,
        Logical, Unary, Variable,
    },
    function::{Callable, CallableInterface, Function, NativeFunction},
    lex::{Literal, TokenType, Tokenizer},
    parser::Parser,
    stmt::{
        Block, Expression, FunctionDecl, If, Print, Return, Stmt, StmtEnum, StmtVisitor, VarDecl,
        While,
    },
};
use anyhow::{bail, Result};

pub struct Interpreter {
    globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new(None)));

        Self {
            globals: Rc::clone(&globals),
            environment: Rc::clone(&globals),
        }
    }

    pub fn define_globals(&mut self, source: String) -> Result<()> {
        let mut tokenizer = Tokenizer::new(source);
        let (tokens, exit_code) = tokenizer.parse();
        if exit_code != 0 {
            bail!("Failed to parse function source");
        }
        let mut parser = Parser::new(tokens);
        let statements = parser.parse()?;

        let old_env = self.environment.clone();
        self.environment = Rc::clone(&self.globals);
        let r = self.interpret(&statements);
        self.environment = old_env;
        r
    }

    pub fn define_native_function(&mut self, name: String, func: fn(Vec<Value>) -> Result<Value>) {
        self.globals.borrow_mut().define(
            name.clone(),
            Value::Callable(Callable::NativeFunction(NativeFunction {
                name,
                arity: 0,
                func,
            })),
        );
    }

    pub fn interpret(&mut self, statements: &[StmtEnum]) -> Result<()> {
        for stmt in statements {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn evaluate(&mut self, expr: &ExprEnum) -> Result<Value> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &dyn Stmt) -> Result<()> {
        stmt.accept(self)
    }

    pub fn execute_block(&mut self, block: &Block, new_env: Environment) -> Result<()> {
        let old_env = self.environment.clone();
        self.environment = Rc::new(RefCell::new(new_env));
        let r = block.statements.iter().try_for_each(|s| {
            let r = s.accept(self);
            r
        });
        self.environment = old_env;
        r
    }
}

impl ExprVisitor for Interpreter {
    type Output = Result<Value>;

    fn visit_binary(&mut self, expr: &Binary) -> Self::Output {
        let right = self.evaluate(expr.right.as_ref())?;
        let left = self.evaluate(expr.left.as_ref())?;

        match expr.operator.token_type {
            TokenType::Plus => match (left, right) {
                (Value::Literal(Literal::Number(left)), Value::Literal(Literal::Number(right))) => {
                    Ok(Value::Literal(Literal::Number(left + right)))
                }
                (Value::Literal(Literal::String(left)), Value::Literal(Literal::String(right))) => {
                    Ok(Value::Literal(Literal::String(left + &right)))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be two numbers or two strings.".into(),
                )),
            },
            TokenType::Minus => match (left, right) {
                (Value::Literal(Literal::Number(left)), Value::Literal(Literal::Number(right))) => {
                    Ok(Value::Literal(Literal::Number(left - right)))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be a numbers.".into(),
                )),
            },
            TokenType::Slash => match (left, right) {
                (Value::Literal(Literal::Number(left)), Value::Literal(Literal::Number(right))) => {
                    Ok(Value::Literal(Literal::Number(left / right)))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be a number.".into(),
                )),
            },
            TokenType::Star => match (left, right) {
                (Value::Literal(Literal::Number(left)), Value::Literal(Literal::Number(right))) => {
                    Ok(Value::Literal(Literal::Number(left * right)))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be a number.".into(),
                )),
            },
            TokenType::Greater => match (left, right) {
                (Value::Literal(Literal::Number(left)), Value::Literal(Literal::Number(right))) => {
                    Ok(Value::Literal(Literal::Boolean(left > right)))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be numbers.".into(),
                )),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Value::Literal(Literal::Number(left)), Value::Literal(Literal::Number(right))) => {
                    Ok(Value::Literal(Literal::Boolean(left >= right)))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be numbers.".into(),
                )),
            },
            TokenType::Less => match (left, right) {
                (Value::Literal(Literal::Number(left)), Value::Literal(Literal::Number(right))) => {
                    Ok(Value::Literal(Literal::Boolean(left < right)))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be numbers.".into(),
                )),
            },
            TokenType::LessEqual => match (left, right) {
                (Value::Literal(Literal::Number(left)), Value::Literal(Literal::Number(right))) => {
                    Ok(Value::Literal(Literal::Boolean(left <= right)))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be numbers.".into(),
                )),
            },
            TokenType::EqualEqual => match (left, right) {
                (Value::Literal(left), Value::Literal(right)) => {
                    Ok(Value::Literal(Literal::Boolean(left.is_equal(&right))))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be two values.".into(),
                )),
            },
            TokenType::BangEqual => match (left, right) {
                (Value::Literal(left), Value::Literal(right)) => {
                    Ok(Value::Literal(Literal::Boolean(!left.is_equal(&right))))
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be two values.".into(),
                )),
            },
            TokenType::And => match left {
                Value::Literal(left) => {
                    if !left.is_truthy() {
                        Ok(Value::Literal(Literal::Boolean(false)))
                    } else {
                        self.evaluate(expr.right.as_ref())
                    }
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be a boolean.".into(),
                )),
            },
            TokenType::Or => match &left {
                Value::Literal(l) => {
                    if l.is_truthy() {
                        Ok(left)
                    } else {
                        self.evaluate(expr.right.as_ref())
                    }
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be a boolean.".into(),
                )),
            },
            _ => bail!(RuntimeError::ParseError(
                expr.operator.clone(),
                "Unknown operator.".into(),
            )),
        }
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> Self::Output {
        self.evaluate(expr.expression.as_ref())
    }

    fn visit_literal(&mut self, expr: &ExprLiteral) -> Self::Output {
        Ok(Value::Literal(expr.value.clone()))
    }

    fn visit_unary(&mut self, expr: &Unary) -> Self::Output {
        let right = self.evaluate(expr.right.as_ref())?;

        match expr.operator.token_type {
            TokenType::Minus => match right {
                Value::Literal(Literal::Number(d)) => Ok(Value::Literal(Literal::Number(-d))),
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be a number.".into(),
                )),
            },
            TokenType::Bang => match right {
                Value::Literal(l) => Ok(Value::Literal(Literal::Boolean(!l.is_truthy()))),
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be a boolean.".into(),
                )),
            },
            _ => bail!(RuntimeError::ParseError(
                expr.operator.clone(),
                "Unknown unary operator.".into(),
            )),
        }
    }

    fn visit_variable(&mut self, expr: &Variable) -> Self::Output {
        let value = self.environment.borrow().get(&expr.name.lexeme);
        match value {
            Some(v) => Ok(v.clone()),
            None => bail!(RuntimeError::ParseError(
                expr.name.clone(),
                format!("Undefined variable '{}'", expr.name.lexeme)
            )),
        }
    }

    fn visit_assignment(&mut self, expr: &Assignment) -> Self::Output {
        let name = &expr.name;
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow_mut()
            .assign(name.lexeme.clone(), value.clone())
            .map_err(|e| RuntimeError::ParseError(name.clone(), e.to_string()))?;
        Ok(value)
    }

    fn visit_logical(&mut self, expr: &Logical) -> Self::Output {
        let left = self.evaluate(&expr.left)?;
        match expr.operator.token_type {
            TokenType::Or => match left {
                Value::Literal(left) => {
                    if left.is_truthy() {
                        // 短路操作
                        Ok(Value::Literal(Literal::Boolean(true)))
                    } else {
                        // 返回右边的值
                        self.evaluate(&expr.right)
                    }
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be a boolean.".into(),
                )),
            },
            TokenType::And => match left {
                Value::Literal(left) => {
                    if !left.is_truthy() {
                        // 短路操作
                        Ok(Value::Literal(Literal::Boolean(false)))
                    } else {
                        // 返回右边的值
                        self.evaluate(&expr.right)
                    }
                }
                _ => bail!(RuntimeError::ParseError(
                    expr.operator.clone(),
                    "Operand must be a boolean.".into(),
                )),
            },
            _ => bail!(RuntimeError::ParseError(
                expr.operator.clone(),
                "Unknown logical operator.".into(),
            )),
        }
    }

    fn visit_call(&mut self, expr: &Call) -> Self::Output {
        let callee = self.evaluate(expr.callee.as_ref())?;

        if let Value::Callable(func) = callee {
            if func.arity() != expr.arguments.len() {
                bail!(RuntimeError::ParseError(
                    expr.paren.clone(),
                    format!(
                        "Expected {} arguments but got {}.",
                        func.arity(),
                        expr.arguments.len()
                    ),
                ));
            }
            let arguments = expr
                .arguments
                .iter()
                .map(|e| self.evaluate(e))
                .collect::<Result<Vec<_>>>()?;
            func.call(self, arguments)
        } else {
            bail!(RuntimeError::ParseError(
                expr.paren.clone(),
                "Can only call functions and classes.".into(),
            ))
        }
    }
}

impl StmtVisitor for Interpreter {
    fn visit_expression(&mut self, stmt: &Expression) -> Result<()> {
        self.evaluate(stmt.expression.as_ref())?;
        Ok(())
    }

    fn visit_print(&mut self, stmt: &Print) -> Result<()> {
        let value = self.evaluate(stmt.expression.as_ref())?;
        println!("{}", value.to_string());
        Ok(())
    }

    fn visit_var_decl(&mut self, stmt: &VarDecl) -> Result<()> {
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
                    .define(stmt.name.lexeme.clone(), Value::Literal(Literal::Nil))
            }
        }
        Ok(())
    }

    fn visit_block(&mut self, stmt: &Block) -> Result<()> {
        let new_env = {
            let env = Environment::new(Some(self.environment.clone()));
            env
        };
        self.execute_block(stmt, new_env)
    }

    fn visit_if(&mut self, stmt: &If) -> Result<()> {
        let condition = self.evaluate(stmt.condition.as_ref())?;
        match condition {
            Value::Literal(literal) => {
                if literal.is_truthy() {
                    self.execute(stmt.then_branch.as_ref())?;
                } else if let Some(else_branch) = stmt.else_branch.as_ref() {
                    self.execute(else_branch.as_ref())?;
                }
            }
            _ => bail!("Condition must be a literal value."),
        }
        Ok(())
    }

    fn visit_while(&mut self, stmt: &While) -> Result<()> {
        while self
            .evaluate(stmt.condition.as_ref())?
            .as_literal()?
            .is_truthy()
        {
            self.execute(stmt.body.as_ref())?;
        }
        Ok(())
    }

    fn visit_function_decl(&mut self, stmt: &FunctionDecl) -> Result<()> {
        let function = Function::new(stmt.clone());
        self.environment.borrow_mut().define(
            stmt.name.lexeme.clone(),
            Value::Callable(Callable::Function(function)),
        );
        Ok(())
    }

    fn visit_return(&mut self, stmt: &Return) -> Result<()> {
        let value = stmt
            .value
            .as_ref()
            .map(|expr| self.evaluate(expr))
            .transpose()?;
        match value {
            Some(value) => bail!(ReturnValue(value)),
            None => bail!(ReturnValue(Value::Literal(Literal::Nil))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lex::Tokenizer, parser::Parser};

    use super::*;

    #[test]
    fn test_nested_blocks() {
        let source = r#"
        fun foo() {
            print "foo";
        }
        foo();
        "#;

        let mut tokenizer = Tokenizer::new(source.to_string());
        let (tokens, exit_code) = tokenizer.parse();
        assert_eq!(exit_code, 0);
        let mut parser = Parser::new(tokens);
        let statements = parser.parse();
        assert!(statements.is_ok());
        let mut interpreter = Interpreter::new();
        let r = interpreter.interpret(&statements.unwrap());
        println!("{:?}", r);
        assert!(r.is_ok());
    }
}
