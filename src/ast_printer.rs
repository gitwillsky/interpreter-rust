use crate::expr::{
    Assignment, Binary, Call, Expr, ExprEnum, ExprVisitor, Grouping, Literal, Logical, Unary,
    Variable,
};
use crate::lex::Literal as LexLiteral;

pub struct AstPrinter {}

impl ExprVisitor for AstPrinter {
    type Output = String;

    fn visit_binary(&mut self, expr: &Binary) -> Self::Output {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> Self::Output {
        self.parenthesize("group", &[&expr.expression])
    }

    fn visit_literal(&mut self, expr: &Literal) -> Self::Output {
        match &expr.value {
            LexLiteral::Nil => "nil".to_string(),
            _ => expr.value.to_string(),
        }
    }

    fn visit_unary(&mut self, expr: &Unary) -> Self::Output {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }

    fn visit_variable(&mut self, expr: &Variable) -> Self::Output {
        expr.name.lexeme.clone()
    }

    fn visit_assignment(&mut self, _expr: &Assignment) -> Self::Output {
        todo!()
    }

    fn visit_logical(&mut self, _expr: &Logical) -> Self::Output {
        todo!()
    }

    fn visit_call(&mut self, _expr: &Call) -> Self::Output {
        todo!()
    }
}

impl AstPrinter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn print(&mut self, expr: &ExprEnum) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Box<ExprEnum>]) -> String {
        let mut str = String::new();

        str.push_str("(");
        str.push_str(name);

        exprs.iter().for_each(|expr| {
            str.push_str(" ");
            str.push_str(&expr.accept(self));
        });

        str.push_str(")");

        str
    }
}
