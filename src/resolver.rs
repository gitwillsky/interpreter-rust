use std::collections::HashMap;

use lox_macro::New;

use crate::{
    error::Error,
    expr::{self, Expr, ExprVisitor},
    interpreter::Interpreter,
    lex,
    stmt::{self, Stmt, StmtVisitor},
};

#[derive(Debug)]
pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            scopes: vec![],
        }
    }

    fn resolve_statements(&mut self, statements: &[stmt::StmtEnum]) -> Result<(), Error> {
        for stmt in statements {
            stmt.accept(self)?;
        }
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &expr::ExprEnum) -> Result<(), Error> {
        expr.accept(self)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &lex::Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &lex::Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_local(&mut self, name: &lex::Token) -> Result<(), Error> {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(name, i);
            }
        }
        Ok(())
    }

    fn resolve_function(&mut self, stmt: &stmt::FunctionDecl) -> Result<(), Error> {
        self.begin_scope();
        for param in &stmt.parameters {
            self.declare(&param);
            self.define(&param);
        }
        self.resolve_statements(&stmt.body.statements)?;
        self.end_scope();
        Ok(())
    }
}

impl ExprVisitor for Resolver {
    type Output = Result<(), Error>;

    fn visit_binary(&mut self, expr: &expr::Binary) -> Self::Output {
        todo!()
    }

    fn visit_grouping(&mut self, expr: &expr::Grouping) -> Self::Output {
        todo!()
    }

    fn visit_literal(&mut self, expr: &expr::Literal) -> Self::Output {
        todo!()
    }

    fn visit_unary(&mut self, expr: &expr::Unary) -> Self::Output {
        todo!()
    }

    fn visit_variable(&mut self, expr: &expr::Variable) -> Self::Output {
        if let Some(scope) = self.scopes.last() {
            // 判断已经在当前作用域声明但未定义
            if let Some(is_defined) = scope.get(&expr.name.lexeme) {
                if !is_defined {
                    return Err(Error::ParseError(
                        expr.name.clone(),
                        "Can't read local variable in its own initializer".to_string(),
                    ));
                }
            }
        }

        self.resolve_local(&expr.name)?;
        Ok(())
    }

    fn visit_assignment(&mut self, expr: &expr::Assignment) -> Self::Output {
        self.resolve_expr(&expr.value)?;
        self.resolve_local(&expr.name)?;
        Ok(())
    }

    fn visit_logical(&mut self, expr: &expr::Logical) -> Self::Output {
        todo!()
    }

    fn visit_call(&mut self, expr: &expr::Call) -> Self::Output {
        todo!()
    }
}

impl StmtVisitor for Resolver {
    type Output = Result<(), Error>;

    fn visit_expression(&mut self, stmt: &stmt::Expression) -> Self::Output {
        todo!()
    }

    fn visit_print(&mut self, stmt: &stmt::Print) -> Self::Output {
        todo!()
    }

    fn visit_var_decl(&mut self, stmt: &stmt::VarDecl) -> Self::Output {
        self.declare(&stmt.name);
        if stmt.initializer.is_some() {
            self.resolve_expr(&stmt.initializer.as_ref().unwrap())?;
        }
        self.define(&stmt.name);
        Ok(())
    }

    fn visit_block(&mut self, stmt: &stmt::Block) -> Self::Output {
        self.begin_scope();
        self.resolve_statements(&stmt.statements)?;
        self.end_scope();
        Ok(())
    }

    fn visit_if(&mut self, stmt: &stmt::If) -> Self::Output {
        todo!()
    }

    fn visit_while(&mut self, stmt: &stmt::While) -> Self::Output {
        todo!()
    }

    fn visit_function_decl(&mut self, stmt: &stmt::FunctionDecl) -> Self::Output {
        self.declare(&stmt.name);
        self.define(&stmt.name);
        self.resolve_function(&stmt);
        Ok(())
    }

    fn visit_return(&mut self, stmt: &stmt::Return) -> Self::Output {
        todo!()
    }
}
