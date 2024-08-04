use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_owned(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;

            self.scan_single_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_owned(),
            literal: None,
            line: self.line,
        })
    }

    fn scan_single_token(&mut self) {
        unimplemented!()
    }

    fn is_at_end(&self) -> bool {
        unimplemented!()
    }

    fn scan_token() {
        unimplemented!()
    }
}
