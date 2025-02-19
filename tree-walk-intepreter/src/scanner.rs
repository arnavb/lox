use crate::token::{Literal, Token, TokenType};

pub struct Scanner<'a> {
    source: Vec<u8>,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.as_bytes().to_owned(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&'a mut self) {
        while !self.is_at_end() {
            //self.start = self.current;

            let next_character = self.advance();
            self.tokens.push(self.character_to_token(next_character));
        }

        //self.tokens.push(Token {
        //    token_type: TokenType::Eof,
        //    lexeme: b"",
        //    literal: None,
        //    line: self.line,
        //})
    }

    fn character_to_token(&'a self, next_character: char) -> Token<'a> {
        match next_character {
            '(' => self.create_token_object(TokenType::LeftParen, None),
            ')' => self.create_token_object(TokenType::RightParen, None),
            '{' => self.create_token_object(TokenType::LeftBrace, None),
            '}' => self.create_token_object(TokenType::RightBrace, None),
            ',' => self.create_token_object(TokenType::Comma, None),
            '.' => self.create_token_object(TokenType::Dot, None),
            '-' => self.create_token_object(TokenType::Minus, None),
            '+' => self.create_token_object(TokenType::Plus, None),
            ';' => self.create_token_object(TokenType::Semicolon, None),
            '*' => self.create_token_object(TokenType::Star, None),
            _ => panic!("Not handled for now"),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        todo!()
    }

    fn create_token_object(&'a self, token_type: TokenType, literal: Option<Literal>) -> Token<'a> {
        let text = &self.source[self.start..self.current];

        Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
        }
    }
}
