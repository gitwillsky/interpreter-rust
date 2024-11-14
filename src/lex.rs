use std::fmt;


pub enum TokenType {
    Var,
    Identifier,
    Equal,
    String,
    Semicolon,
    Eof,
    LeftParen,
    RightParen,

    Unknown,
}

impl TokenType {
    pub fn as_str(&self) -> &str {
        match self {
            TokenType::Var => "VAR",
            TokenType::Identifier => "IDENTIFIER",
            TokenType::Equal => "EQUAL",
            TokenType::String => "STRING",
            TokenType::Semicolon => "SEMICOLON",
            TokenType::LeftParen => "LEFT_PAREN",
            TokenType::RightParen => "RIGHT_PAREN",
            TokenType::Eof => "EOF",
            TokenType::Unknown => "UNKNOWN",
        }
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
            self.token_type.as_str(),
            self.lexeme,
            self.literal.as_ref().unwrap_or(&String::from("null"))
        )
    }
}

pub struct Tokenizer {
    // TODO use bufReader instead of
    source: String,
    current_line: usize,
    current_column: usize,
    offset: usize,
}

impl Tokenizer {
    pub fn new(source: String) -> Self {
        Self {
            source,
            current_line: 1,
            current_column: 1,
            offset: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(c) = self.advance() {
            let token = match c {
                '(' => Token::new(TokenType::LeftParen, c.into(), None),
                ')' => Token::new(TokenType::RightParen, c.into(), None),
                _ => Token::new(TokenType::Unknown, c.into(), None)
            };
            tokens.push(token);
        }
        tokens.push(Token::new(TokenType::Eof, "".into(), None));
        tokens
    }

    /// return the next char
    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.offset);
        if c.is_some() {
            self.offset += 1;
        }
        c
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_parse() {
//         let s = "";

//     }
// }