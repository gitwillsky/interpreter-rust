use crate::{
    error::Error,
    expr::{Assignment, Binary, Call, ExprEnum, Grouping, Literal as ExprLiteral, Unary, Variable},
    lex::{Literal, Token, TokenType},
    stmt::{Block, Expression, FunctionDecl, If, Print, Return, StmtEnum, VarDecl, While},
};

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

    fn consume(
        &mut self,
        token_type: TokenType,
        message: impl Into<String>,
    ) -> Result<&Token, Error> {
        if self.check_token(token_type) {
            Ok(self.advance())
        } else {
            return Err(Error::ParseError(self.peek().clone(), message.into()));
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
 * program        → declaration* EOF ;
 * declaration    → var_decl | fun_decl | statement ;
 * var_decl       → "var" IDENTIFIER ( "=" expression )? ";" ;
 * fun_decl       → "fun" function ;
 * function       → IDENTIFIER "(" parameters? ")" block ;
 * parameters     → IDENTIFIER ( "," IDENTIFIER )* ;
 * statement      → expr_stmt | for_stmt | if_stmt | print_stmt | return_stmt | while_stmt | block ;
 * for_stmt       → "for" "(" ( var_decl | expr_stmt | ";" ) expression? ";" expression? ")" statement ;
 * if_stmt        → "if" "(" expression ")" statement ( "else" statement )? ;
 * while_stmt     → "while" "(" expression ")" statement ;
 * block          → "{" declaration* "}" ;
 * expr_stmt      → expression ";";
 * print_stmt     → "print" expression ";";
 * return_stmt    → "return" expression? ";";
 * expression     → assignment;
 * assignment     → IDENTIFIER "=" assignment | logic_or;
 * logic_or       → logic_and ( "or" logic_and )* ;
 * logic_and      → equality ( "and" equality )* ;
 * equality       → comparison ( ( "!=" | "==" ) comparison )* ;
 * comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
 * term           → factor ( ( "-" | "+" ) factor )* ;
 * factor         → unary ( ( "/" | "*" ) unary )* ;
 * unary          → ( "!" | "-" ) unary | call ;
 * call           → primary ( "(" arguments? ")" )* ;
 * arguments      → expression ( "," expression )* ;
 * primary        → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;
 */
impl Parser {
    pub fn parse(&mut self) -> Result<Vec<StmtEnum>, Error> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
            // match self.declaration() {
            //     Ok(stmt) => statements.push(stmt),
            //     Err(_) => {
            //         self.synchronize();
            //     }
            // }
        }

        Ok(statements)
    }

    fn return_stmt(&mut self) -> Result<StmtEnum, Error> {
        let keyword = self.previous().clone();
        let value = if !self.check_token(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expected ';' after return value.")?;
        Ok(StmtEnum::Return(Return::new(keyword, value.map(Box::new))))
    }

    fn function(&mut self, kind: String) -> Result<StmtEnum, Error> {
        let name = self
            .consume(TokenType::Identifier, format!("Expected {} name.", kind))?
            .clone();
        self.consume(
            TokenType::LeftParen,
            format!("Expected '(' after {} name.", kind),
        )?;

        let mut parameters = Vec::new();
        if !self.check_token(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err(Error::ParseError(
                        self.peek().clone(),
                        "Can't have more than 255 parameters.".into(),
                    ));
                }
                parameters.push(
                    self.consume(TokenType::Identifier, "Expected parameter name.")
                        .map(|token| token.clone())?,
                );
                if !self.match_token(TokenType::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expected ')' after parameters.")?;
        self.consume(
            TokenType::LeftBrace,
            format!("Expected '{{' before {} body.", kind),
        )?;
        Ok(StmtEnum::FunctionDecl(FunctionDecl::new(
            name,
            parameters,
            self.block()?,
        )))
    }

    fn call(&mut self) -> Result<ExprEnum, Error> {
        let mut expr = self.primary()?;
        while self.match_token(TokenType::LeftParen) {
            expr = self.finish_call(expr)?;
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: ExprEnum) -> Result<ExprEnum, Error> {
        let mut arguments = Vec::new();
        if !self.check_token(TokenType::RightParen) {
            arguments.push(self.expression()?);
            while self.match_token(TokenType::Comma) {
                if arguments.len() >= 255 {
                    return Err(Error::ParseError(
                        self.peek().clone(),
                        "Can't have more than 255 arguments.".into(),
                    ));
                }
                arguments.push(self.expression()?);
            }
        }
        let right_paren = self.consume(TokenType::RightParen, "Expected ')' after arguments.")?;

        Ok(ExprEnum::Call(Call::new(
            Box::new(callee),
            right_paren.clone(),
            arguments,
        )))
    }

    fn while_stmt(&mut self) -> Result<StmtEnum, Error> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after condition.")?;
        let body = self.statement()?;
        Ok(StmtEnum::While(While::new(
            Box::new(condition),
            Box::new(body),
        )))
    }

    fn for_stmt(&mut self) -> Result<StmtEnum, Error> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'for'.")?;

        let initializer = if self.match_token(TokenType::Var) {
            Some(self.var_decl()?)
        } else if self.match_token(TokenType::Semicolon) {
            None
        } else {
            Some(self.expr_stmt()?)
        };

        let condition = if self.match_token(TokenType::Semicolon) {
            None
        } else {
            let expr = self.expression()?;
            self.consume(TokenType::Semicolon, "Expected ';' after expression.")?;
            Some(expr)
        };

        let increment = if self.match_token(TokenType::RightParen) {
            None
        } else {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after for clauses.")?;
            Some(expr)
        };

        let mut body = self.statement()?;
        if let Some(increment) = increment {
            body = StmtEnum::Block(Block::new(vec![
                body,
                StmtEnum::Expression(Expression::new(Box::new(increment))),
            ]));
        }

        body = StmtEnum::While(While::new(
            Box::new(
                condition.unwrap_or(ExprEnum::Literal(ExprLiteral::new(Literal::Boolean(true)))),
            ),
            Box::new(body),
        ));

        if let Some(initializer) = initializer {
            body = StmtEnum::Block(Block::new(vec![initializer, body]));
        }

        Ok(body)
    }

    fn declaration(&mut self) -> Result<StmtEnum, Error> {
        if self.match_token(TokenType::Var) {
            self.var_decl()
        } else if self.match_token(TokenType::Fun) {
            self.function("function".to_string())
        } else {
            self.statement()
        }
    }

    fn var_decl(&mut self) -> Result<StmtEnum, Error> {
        let name = self
            .consume(TokenType::Identifier, "Expected variable name.")?
            .clone();

        let initializer = if self.match_token(TokenType::Equal) {
            Some(Box::new(self.expression()?))
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration.",
        )?;

        Ok(StmtEnum::VarDecl(VarDecl::new(name, initializer)))
    }

    fn statement(&mut self) -> Result<StmtEnum, Error> {
        if self.match_token(TokenType::Print) {
            self.print_stmt()
        } else if self.match_token(TokenType::LeftBrace) {
            Ok(StmtEnum::Block(self.block()?))
        } else if self.match_token(TokenType::If) {
            self.if_stmt()
        } else if self.match_token(TokenType::While) {
            self.while_stmt()
        } else if self.match_token(TokenType::For) {
            self.for_stmt()
        } else if self.match_token(TokenType::Return) {
            self.return_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn if_stmt(&mut self) -> Result<StmtEnum, Error> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after condition.")?;
        let then_branch = self.statement()?;
        let else_branch = if self.match_token(TokenType::Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(StmtEnum::If(If::new(
            Box::new(condition),
            Box::new(then_branch),
            else_branch,
        )))
    }

    fn block(&mut self) -> Result<Block, Error> {
        let mut statements = Vec::new();
        while !self.check_token(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block")?;
        Ok(Block::new(statements))
    }

    fn print_stmt(&mut self) -> Result<StmtEnum, Error> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value.")?;
        Ok(StmtEnum::Print(Print::new(Box::new(expr))))
    }

    fn expr_stmt(&mut self) -> Result<StmtEnum, Error> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression.")?;
        Ok(StmtEnum::Expression(Expression::new(Box::new(expr))))
    }

    pub fn expression(&mut self) -> Result<ExprEnum, Error> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<ExprEnum, Error> {
        let expr = self.logic_or()?;

        if self.match_token(TokenType::Equal) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            return match expr {
                ExprEnum::Variable(variable) => Ok(ExprEnum::Assignment(Assignment::new(
                    variable.name,
                    Box::new(value),
                ))),
                _ => Err(Error::ParseError(
                    equals.clone(),
                    "Invalid assignment target.".into(),
                )),
            };
        }

        Ok(expr)
    }

    fn logic_or(&mut self) -> Result<ExprEnum, Error> {
        let mut expr = self.logic_and();

        while self.match_token(TokenType::Or) {
            let operator = self.previous().clone();
            let right = self.logic_and()?;
            expr = Ok(ExprEnum::Binary(Binary::new(
                Box::new(expr?),
                operator,
                Box::new(right),
            )));
        }

        expr
    }

    fn logic_and(&mut self) -> Result<ExprEnum, Error> {
        let mut expr = self.equality()?;

        while self.match_token(TokenType::And) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = ExprEnum::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<ExprEnum, Error> {
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

    fn comparison(&mut self) -> Result<ExprEnum, Error> {
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

    fn term(&mut self) -> Result<ExprEnum, Error> {
        let mut expr = self.factor()?;

        while self.match_token(TokenType::Minus) || self.match_token(TokenType::Plus) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = ExprEnum::Binary(Binary::new(Box::new(expr), operator, Box::new(right)));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<ExprEnum, Error> {
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

    fn unary(&mut self) -> Result<ExprEnum, Error> {
        if self.match_token(TokenType::Bang) || self.match_token(TokenType::Minus) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(ExprEnum::Unary(Unary::new(operator, Box::new(right))));
        }

        self.call()
    }

    fn primary(&mut self) -> Result<ExprEnum, Error> {
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
                Ok(ExprEnum::Grouping(Grouping::new(Box::new(expr?))))
            }
            TokenType::Identifier => Ok(ExprEnum::Variable(Variable::new(token.clone()))),
            _ => Err(Error::ParseError(
                token.clone(),
                format!("Expected expression, got {}", token.lexeme),
            )),
        }
    }
}
