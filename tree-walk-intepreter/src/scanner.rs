use crate::token::{Literal, Token, TokenType};

pub struct Scanner<'source> {
    source: &'source [u8],
    tokens: Vec<Token<'source>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source: source.as_bytes(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&'source mut self) {
        while !self.is_at_end() {
            self.start = self.current;

            let next_token = self.scan_to_token();
            self.tokens.push(next_token);
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: b"",
            literal: None,
            line: self.line,
        })
    }

    fn scan_to_token(&mut self) -> Token<'source> {
        let next_character = self.advance();

        match next_character {
            b'(' => self.create_token_object(TokenType::LeftParen, None),
            b')' => self.create_token_object(TokenType::RightParen, None),
            b'{' => self.create_token_object(TokenType::LeftBrace, None),
            b'}' => self.create_token_object(TokenType::RightBrace, None),
            b',' => self.create_token_object(TokenType::Comma, None),
            b'.' => self.create_token_object(TokenType::Dot, None),
            b'-' => self.create_token_object(TokenType::Minus, None),
            b'+' => self.create_token_object(TokenType::Plus, None),
            b';' => self.create_token_object(TokenType::Semicolon, None),
            b'*' => self.create_token_object(TokenType::Star, None),
            _ => panic!("Not handled for now"),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> u8 {
        let result = *self
            .source
            .get(self.current)
            .expect("out of range scanner index (this should never happen)");

        self.current += 1;

        result
    }

    fn create_token_object(
        &self,
        token_type: TokenType,
        literal: Option<Literal>,
    ) -> Token<'source> {
        let text = &self.source[self.start..self.current];

        Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
        }
    }
}
