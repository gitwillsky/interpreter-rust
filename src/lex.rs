use std::fmt;

use log::error;

#[derive(PartialEq)]
pub enum TokenType {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // keywords
    Var,

    Eof,
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::Var => "VAR",
            TokenType::Identifier => "IDENTIFIER",
            TokenType::Equal => "EQUAL",
            TokenType::String => "STRING",
            TokenType::Semicolon => "SEMICOLON",
            TokenType::LeftParen => "LEFT_PAREN",
            TokenType::RightParen => "RIGHT_PAREN",
            TokenType::LeftBrace => "LEFT_BRACE",
            TokenType::RightBrace => "RIGHT_BRACE",
            TokenType::Eof => "EOF",
            TokenType::Star => "STAR",
            TokenType::Dot => "DOT",
            TokenType::Comma => "COMMA",
            TokenType::Plus => "PLUS",
            TokenType::Minus => "MINUS",
            TokenType::EqualEqual => "EQUAL_EQUAL",
            TokenType::Bang => "BANG",
            TokenType::BangEqual => "BANG_EQUAL",
            TokenType::Greater => "GREATER",
            TokenType::GreaterEqual => "GREATER_EQUAL",
            TokenType::Less => "LESS",
            TokenType::LessEqual => "LESS_EQUAL",
            TokenType::Slash => "SLASH",
            TokenType::Number => "NUMBER",
        }
        .into()
    }
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
}

pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match self {
            Literal::String(s) => s.clone(),
            Literal::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{:.1}", n)
                } else {
                    n.to_string()
                }
            }
            Literal::Boolean(b) => b.to_string(),
            Literal::Nil => "null".into(),
        }
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type.to_string(),
            self.lexeme,
            self.literal.as_ref().unwrap_or(&Literal::Nil).to_string(),
        )
    }
}

pub struct Tokenizer {
    line_number: usize,
    source: Vec<char>,
    start: usize,
    current: usize,
}

impl Tokenizer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line_number: 1,
        }
    }

    pub fn parse(&mut self) -> (Vec<Token>, i32) {
        let mut tokens = Vec::new();
        let mut exit_code = 0;
        while let Some(c) = self.advance() {
            // skip new line
            if matches!(c, '\n') {
                self.line_number += 1;
                self.start = self.current;
                continue;
            }
            if c.is_whitespace() {
                self.start = self.current;
                continue;
            }
            let token = match c {
                '(' => Some(Token::new(TokenType::LeftParen, c.into(), None)),
                ')' => Some(Token::new(TokenType::RightParen, c.into(), None)),
                '{' => Some(Token::new(TokenType::LeftBrace, c.into(), None)),
                '}' => Some(Token::new(TokenType::RightBrace, c.into(), None)),
                '*' => Some(Token::new(TokenType::Star, c.into(), None)),
                '.' => Some(Token::new(TokenType::Dot, c.into(), None)),
                ',' => Some(Token::new(TokenType::Comma, c.into(), None)),
                '+' => Some(Token::new(TokenType::Plus, c.into(), None)),
                '-' => Some(Token::new(TokenType::Minus, c.into(), None)),
                ';' => Some(Token::new(TokenType::Semicolon, c.into(), None)),
                '=' => match self.peek() {
                    Some('=') => {
                        // 已经消费了，offset + 1
                        self.current += 1;
                        Some(Token::new(TokenType::EqualEqual, "==".into(), None))
                    }
                    _ => Some(Token::new(TokenType::Equal, c.into(), None)),
                },
                '!' => match self.peek() {
                    Some('=') => {
                        self.current += 1;
                        Some(Token::new(TokenType::BangEqual, "!=".into(), None))
                    }
                    _ => Some(Token::new(TokenType::Bang, c.into(), None)),
                },
                '<' => match self.peek() {
                    Some('=') => {
                        self.current += 1;
                        Some(Token::new(TokenType::LessEqual, "<=".into(), None))
                    }
                    _ => Some(Token::new(TokenType::Less, c.into(), None)),
                },
                '>' => match self.peek() {
                    Some('=') => {
                        self.current += 1;
                        Some(Token::new(TokenType::GreaterEqual, ">=".into(), None))
                    }
                    _ => Some(Token::new(TokenType::Greater, c.into(), None)),
                },
                '/' => match self.peek() {
                    Some('/') => {
                        self.current += 1;
                        while let Some(c) = self.peek() {
                            match c {
                                '\n' => break,
                                _ => self.current += 1,
                            }
                        }
                        continue;
                    }
                    _ => Some(Token::new(TokenType::Slash, c.into(), None)),
                },
                '"' => {
                    let mut has_terminated = false;
                    while let Some(c) = self.advance() {
                        if c == '"' {
                            has_terminated = true;
                            break;
                        }
                    }
                    if !has_terminated {
                        error!("[line {}] Error: Unterminated string.", self.line_number);
                        None
                    } else {
                        // ignore double quote
                        let literal: String = self.source[self.start + 1..self.current - 1]
                            .iter()
                            .collect();
                        Some(Token::new(
                            TokenType::String,
                            format!("\"{}\"", literal),
                            Some(Literal::String(literal)),
                        ))
                    }
                }
                '0'..='9' => {
                    while let Some(c) = self.peek() {
                        match c {
                            '0'..='9' => {
                                self.current += 1;
                            }
                            '.' => {
                                if let Some(c) = self.peek_next() {
                                    if c.is_ascii_digit() {
                                        self.current += 2;
                                    } else {
                                        break;
                                    }
                                }
                            }
                            _ => break,
                        }
                    }
                    let literal: String = self.source[self.start..self.current].iter().collect();
                    Some(Token::new(
                        TokenType::Number,
                        literal.clone(),
                        Some(Literal::Number(literal.parse::<f64>().unwrap())),
                    ))
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    while let Some(c) = self.peek() {
                        if !matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9') {
                            break;
                        } else {
                            self.current += 1;
                        }
                    }
                    let literal: String = self.source[self.start..self.current].iter().collect();
                    Some(Token::new(TokenType::Identifier, literal, None))
                }
                _ => {
                    error!(
                        "[line {}] Error: Unexpected character: {}",
                        self.line_number, c
                    );
                    None
                }
            };
            match token {
                Some(t) => tokens.push(t),
                None => {
                    exit_code = 65;
                }
            }
            // update start
            self.start = self.current;
        }
        tokens.push(Token::new(TokenType::Eof, "".into(), None));
        (tokens, exit_code)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// return the next char
    fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        let c = self.source[self.current];
        self.current += 1;
        Some(c)
    }

    /// return the next chart without move current
    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        Some(self.source[self.current])
    }

    /// return the next next char without move current
    fn peek_next(&self) -> Option<char> {
        let next_index = self.current + 1;
        if next_index >= self.source.len() {
            None
        } else {
            Some(self.source[next_index])
        }
    }
}
