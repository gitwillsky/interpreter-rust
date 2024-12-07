use crate::expr::{
    Assignment, Binary, Call, Expr, ExprEnum, ExprVisitor, Grouping, Literal, Logical, Unary,
    Variable,
};
use crate::lex::Literal as LexLiteral;

pub struct AstPrinter {}

impl ExprVisitor for AstPrinter {
    type Output = String;

    fn visit_binary(&self, expr: &Binary) -> Self::Output {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_grouping(&self, expr: &Grouping) -> Self::Output {
        self.parenthesize("group", &[&expr.expression])
    }

    fn visit_literal(&self, expr: &Literal) -> Self::Output {
        match &expr.value {
            LexLiteral::Nil => "nil".to_string(),
            _ => expr.value.to_string(),
        }
    }

    fn visit_unary(&self, expr: &Unary) -> Self::Output {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }

    fn visit_variable(&self, expr: &Variable) -> Self::Output {
        expr.name.lexeme.clone()
    }

    fn visit_assignment(&self, _expr: &Assignment) -> Self::Output {
        todo!()
    }

    fn visit_logical(&self, _expr: &Logical) -> Self::Output {
        todo!()
    }

    fn visit_call(&self, _expr: &Call) -> Self::Output {
        todo!()
    }
}

impl AstPrinter {
    pub fn new() -> Self {
        Self {}
    }
    pub fn print(&self, expr: &ExprEnum) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: &str, exprs: &[&Box<ExprEnum>]) -> String {
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
