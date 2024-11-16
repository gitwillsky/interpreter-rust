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
    literal: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>) -> Self {
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
            self.literal.as_ref().unwrap_or(&String::from("null"))
        )
    }
}

pub struct Tokenizer {
    // TODO use bufReader instead of
    line: usize,
    source: String,
    offset: usize,
}

impl Tokenizer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            offset: 0,
            line: 1,
        }
    }

    pub fn parse(&mut self) -> (Vec<Token>, i32) {
        let mut tokens = Vec::new();
        let mut exit_code = 0;
        while let Some(c) = self.advance() {
            // skip new line
            if matches!(c, '\n') {
                self.line += 1;
                continue;
            }
            if c.is_whitespace() {
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
                        self.offset += 1;
                        Some(Token::new(TokenType::EqualEqual, "==".into(), None))
                    }
                    _ => Some(Token::new(TokenType::Equal, c.into(), None)),
                },
                '!' => match self.peek() {
                    Some('=') => {
                        self.offset += 1;
                        Some(Token::new(TokenType::BangEqual, "!=".into(), None))
                    }
                    _ => Some(Token::new(TokenType::Bang, c.into(), None)),
                },
                '<' => match self.peek() {
                    Some('=') => {
                        self.offset += 1;
                        Some(Token::new(TokenType::LessEqual, "<=".into(), None))
                    }
                    _ => Some(Token::new(TokenType::Less, c.into(), None)),
                },
                '>' => match self.peek() {
                    Some('=') => {
                        self.offset += 1;
                        Some(Token::new(TokenType::GreaterEqual, ">=".into(), None))
                    }
                    _ => Some(Token::new(TokenType::Greater, c.into(), None)),
                },
                '/' => match self.peek() {
                    Some('/') => {
                        self.offset += 1;
                        while let Some(c) = self.peek() {
                            match c {
                                '\n' => break,
                                _ => self.offset += 1,
                            }
                        }
                        continue;
                    }
                    _ => Some(Token::new(TokenType::Slash, c.into(), None)),
                },
                '"' => {
                    let mut has_terminated = false;
                    let mut literal = String::new();
                    while let Some(c) = self.advance() {
                        match c {
                            '"' => {
                                has_terminated = true;
                                break;
                            }
                            _ => literal.push(c),
                        }
                    }
                    if !has_terminated {
                        error!("[line {}] Error: Unterminated string.", self.line);
                        None
                    } else {
                        Some(Token::new(
                            TokenType::String,
                            format!("\"{}\"", literal),
                            Some(literal),
                        ))
                    }
                }
                _ => {
                    error!("[line {}] Error: Unexpected character: {}", self.line, c);
                    None
                }
            };
            match token {
                Some(t) => tokens.push(t),
                None => {
                    exit_code = 65;
                }
            }
        }
        tokens.push(Token::new(TokenType::Eof, "".into(), None));
        (tokens, exit_code)
    }

    /// return the next char
    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.offset);
        if c.is_some() {
            self.offset += 1;
        }
        c
    }

    /// return the next chart without move offset
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.offset)
    }
}
