use std::collections::HashMap;

use crate::{
    error::Error,
    expr::{self, Expr, ExprVisitor},
    interpreter::Interpreter,
    lex,
    stmt::{self, Stmt, StmtVisitor},
};

#[derive(Debug)]
pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter<'a>,
    // 用 Vec 来记录当前作用域的栈，栈中的每个元素代表一个块作用域的 Map
    // 作用域栈只用于局部作用域，解析器不会跟踪全局作用域，因为它们会在运行时动态改变
    // true/false 表示是否已定义
    scopes: Vec<HashMap<String, bool>>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter<'a>) -> Self {
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

    // 开始一个新的块作用域
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

    fn resolve_local(&mut self, name: &'a lex::Token) -> Result<(), Error> {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(name, i);
                return Ok(());
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

impl<'a> ExprVisitor for Resolver<'a> {
    type Output = Result<(), Error>;

    fn visit_binary(&mut self, expr: &expr::Binary) -> Self::Output {
        expr.left.accept(self)?;
        expr.right.accept(self)?;
        Ok(())
    }

    fn visit_grouping(&mut self, expr: &expr::Grouping) -> Self::Output {
        expr.expression.accept(self)?;
        Ok(())
    }

    fn visit_literal(&mut self, _expr: &expr::Literal) -> Self::Output {
        Ok(())
    }

    fn visit_unary(&mut self, expr: &expr::Unary) -> Self::Output {
        expr.right.accept(self)?;
        Ok(())
    }

    fn visit_variable(&mut self, expr: &expr::Variable) -> Self::Output {
        if let Some(scope) = self.scopes.last() {
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
        expr.value.accept(self)?;
        self.resolve_local(&expr.name)?;
        Ok(())
    }

    fn visit_logical(&mut self, expr: &expr::Logical) -> Self::Output {
        expr.left.accept(self)?;
        expr.right.accept(self)?;
        Ok(())
    }

    fn visit_call(&mut self, expr: &expr::Call) -> Self::Output {
        expr.callee.accept(self)?;
        for arg in &expr.arguments {
            arg.accept(self)?;
        }
        Ok(())
    }
}

impl<'a> StmtVisitor for Resolver<'a> {
    type Output = Result<(), Error>;

    fn visit_expression(&mut self, stmt: &stmt::Expression) -> Self::Output {
        stmt.expression.accept(self)?;
        Ok(())
    }

    fn visit_print(&mut self, stmt: &stmt::Print) -> Self::Output {
        stmt.expression.accept(self)?;
        Ok(())
    }

    fn visit_var_decl(&mut self, stmt: &stmt::VarDecl) -> Self::Output {
        self.declare(&stmt.name);
        if let Some(initializer) = &stmt.initializer {
            initializer.accept(self)?;
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
        stmt.condition.accept(self)?;
        stmt.then_branch.accept(self)?;
        if let Some(else_branch) = &stmt.else_branch {
            else_branch.accept(self)?;
        }
        Ok(())
    }

    fn visit_while(&mut self, stmt: &stmt::While) -> Self::Output {
        stmt.condition.accept(self)?;
        stmt.body.accept(self)?;
        Ok(())
    }

    fn visit_function_decl(&mut self, stmt: &stmt::FunctionDecl) -> Self::Output {
        self.declare(&stmt.name);
        self.define(&stmt.name);
        self.resolve_function(&stmt);
        Ok(())
    }

    fn visit_return(&mut self, stmt: &stmt::Return) -> Self::Output {
        if let Some(value) = &stmt.value {
            value.accept(self)?;
        }
        Ok(())
    }
}
