use crate::{
    error::RuntimeError,
    expr::{Binary, ExprEnum, Grouping, Literal as ExprLiteral, Unary},
    lex::{Literal, Token, TokenType},
    stmt::{Expression, Print, StmtEnum},
};

use anyhow::{bail, Result};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check_token(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check_token(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenType, message: impl Into<String>) -> Result<&Token> {
        if self.check_token(token_type) {
            Ok(self.advance())
        } else {
            bail!(RuntimeError::ParseError(
                self.peek().clone(),
                message.into(),
            ))
        }
    }
    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}

/**
 * program        → statement* EOF ;
 * statement      → expr_stmt | print_stmt ;
 * expr_stmt      → expression ";";
 * print_stmt     → "print" expression ";";
 * expression     → equality;
 * equality       → comparison ( ( "!=" | "==" ) comparison )* ;
 * comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
 * term           → factor ( ( "-" | "+" ) factor )* ;
 * factor         → unary ( ( "/" | "*" ) unary )* ;
 * unary          → ( "!" | "-" ) unary | primary ;
 * primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
 */
impl Parser {
    pub fn parse(&mut self) -> Result<Vec<StmtEnum>> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<StmtEnum> {
        if self.match_token(TokenType::Print) {
            self.print_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn print_stmt(&mut self) -> Result<StmtEnum> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value.")?;
        Ok(StmtEnum::Print(Print::new(Box::new(expr))))
    }

    fn expr_stmt(&mut self) -> Result<StmtEnum> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression.")?;
        Ok(StmtEnum::Expression(Expression::new(Box::new(expr))))
    }

    pub fn expression(&mut self) -> Result<ExprEnum> {
        self.equality()
    }

    fn equality(&mut self) -> Result<ExprEnum> {
        let mut expr = self.comparison();

        while self.match_token(TokenType::EqualEqual) || self.match_token(TokenType::BangEqual) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Ok(ExprEnum::Binary(Binary::new(
                Box::new(expr?),
                operator,
                Box::new(right),
            )));
        }

        expr
    }

    fn comparison(&mut self) -> Result<ExprEnum> {
        let mut expr = self.term();

        while self.match_token(TokenType::Greater)
            || self.match_token(TokenType::GreaterEqual)
            || self.match_token(TokenType::Less)
            || self.match_token(TokenType::LessEqual)
        {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Ok(ExprEnum::Binary(Binary::new(
                Box::new(expr?),
                operator,
                Box::new(right),
            )));
        }

        expr
    }

    fn term(&mut self) -> Result<ExprEnum> {
        let mut expr = self.factor();

        while self.match_token(TokenType::Minus) || self.match_token(TokenType::Plus) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Ok(ExprEnum::Binary(Binary::new(
                Box::new(expr?),
                operator,
                Box::new(right),
            )));
        }

        expr
    }

    fn factor(&mut self) -> Result<ExprEnum> {
        let mut expr = self.unary();

        while self.match_token(TokenType::Slash) || self.match_token(TokenType::Star) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Ok(ExprEnum::Binary(Binary::new(
                Box::new(expr?),
                operator,
                Box::new(right),
            )));
        }

        expr
    }

    fn unary(&mut self) -> Result<ExprEnum> {
        if self.match_token(TokenType::Bang) || self.match_token(TokenType::Minus) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(ExprEnum::Unary(Unary::new(operator, Box::new(right))));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<ExprEnum> {
        let token = self.advance();

        match token.token_type {
            TokenType::False => Ok(ExprEnum::Literal(ExprLiteral::new(Literal::Boolean(false)))),
            TokenType::True => Ok(ExprEnum::Literal(ExprLiteral::new(Literal::Boolean(true)))),
            TokenType::Nil => Ok(ExprEnum::Literal(ExprLiteral::new(Literal::Nil))),
            TokenType::Number => Ok(ExprEnum::Literal(ExprLiteral::new(
                token.literal.clone().unwrap(),
            ))),
            TokenType::String => Ok(ExprEnum::Literal(ExprLiteral::new(
                token.literal.clone().unwrap(),
            ))),
            TokenType::LeftParen => {
                let expr = self.expression();
                self.consume(TokenType::RightParen, "Expected ')' after expression")?;
                let expr = ExprEnum::Grouping(Grouping::new(Box::new(expr?)));
                Ok(expr)
            }
            _ => bail!(RuntimeError::ParseError(
                token.clone(),
                format!("Expected expression, got {}", token.lexeme),
            )),
        }
    }
}
