use crate::token::{Literal, Token, TokenType};

pub enum ScanError {
    UnexpectedCharacter(u8),
}

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

    pub fn scan_tokens(&'source mut self) -> Vec<ScanError> {
        let mut scan_errors = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;

            match self.scan_to_token() {
                Ok(next_token) => self.tokens.push(next_token),
                Err(e) => scan_errors.push(e),
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: b"",
            literal: None,
            line: self.line,
        });

        scan_errors
    }

    fn scan_to_token(&mut self) -> Result<Token<'source>, ScanError> {
        let next_character = self.advance();

        match next_character {
            b'(' => Ok(self.create_token_object(TokenType::LeftParen, None)),
            b')' => Ok(self.create_token_object(TokenType::RightParen, None)),
            b'{' => Ok(self.create_token_object(TokenType::LeftBrace, None)),
            b'}' => Ok(self.create_token_object(TokenType::RightBrace, None)),
            b',' => Ok(self.create_token_object(TokenType::Comma, None)),
            b'.' => Ok(self.create_token_object(TokenType::Dot, None)),
            b'-' => Ok(self.create_token_object(TokenType::Minus, None)),
            b'+' => Ok(self.create_token_object(TokenType::Plus, None)),
            b';' => Ok(self.create_token_object(TokenType::Semicolon, None)),
            b'*' => Ok(self.create_token_object(TokenType::Star, None)),

            // Two character lexeme
            b'!' => Ok(self.create_two_character_token_object(
                b'=',
                TokenType::BangEqual,
                TokenType::Bang,
            )),
            b'=' => Ok(self.create_two_character_token_object(
                b'=',
                TokenType::EqualEqual,
                TokenType::Equal,
            )),
            b'>' => Ok(self.create_two_character_token_object(
                b'=',
                TokenType::LessEqual,
                TokenType::Less,
            )),
            b'<' => Ok(self.create_two_character_token_object(
                b'=',
                TokenType::GreaterEqual,
                TokenType::Greater,
            )),

            // Comments or division
            b'/' => {
                if self.match_next(b'/') {
                    while let Some(ch) = self.peek() {
                        if self.is_at_end() || ch == b'\n' {
                            break;
                        }
                        self.advance();
                    }

                    // TODO: Actually store the comment
                    Ok(self.create_token_object(TokenType::SingleLineComment, None))
                } else {
                    Ok(self.create_token_object(TokenType::Slash, None))
                }
            }
            e => Err(ScanError::UnexpectedCharacter(e)),
        }
    }

    fn create_two_character_token_object(
        &mut self,
        expected: u8,
        double: TokenType,
        single: TokenType,
    ) -> Token<'source> {
        let next_token_type = if self.match_next(expected) {
            double
        } else {
            single
        };
        self.create_token_object(next_token_type, None)
    }

    fn peek(&self) -> Option<u8> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source[self.current])
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

    fn match_next(&mut self, expected: u8) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;

        true
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
