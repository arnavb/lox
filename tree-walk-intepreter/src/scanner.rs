use std::{collections::HashMap, str::from_utf8};

use crate::token::{Literal, Token, TokenType};

pub enum ScanError {
    UnexpectedCharacter(u8),
    UnterminatedString,
}

pub struct Scanner<'source> {
    source: &'source [u8],
    pub tokens: Vec<Token<'source>>,
    start: usize,
    current: usize,
    line: usize,

    reserved_keywords: HashMap<&'static str, TokenType>,
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        use TokenType::*;

        Self {
            source: source.as_bytes(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,

            reserved_keywords: HashMap::from([
                ("and", And),
                ("class", Class),
                ("else", Else),
                ("false", False),
                ("for", For),
                ("fun", Fun),
                ("if", If),
                ("nil", Nil),
                ("or", Or),
                ("print", Print),
                ("return", Return),
                ("super", Super),
                ("this", This),
                ("true", True),
                ("var", Var),
                ("while", While),
            ]),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<ScanError> {
        let mut scan_errors = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;

            if let Err(e) = self.scan_next_token() {
                scan_errors.push(e);
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

    fn scan_next_token(&mut self) -> Result<(), ScanError> {
        use TokenType::*;

        let next_character = self.advance();

        match next_character {
            b'(' => Ok(self.push_single_character_token_object(LeftParen)),
            b')' => Ok(self.push_single_character_token_object(RightParen)),
            b'{' => Ok(self.push_single_character_token_object(LeftBrace)),
            b'}' => Ok(self.push_single_character_token_object(RightBrace)),
            b',' => Ok(self.push_single_character_token_object(Comma)),
            b'.' => Ok(self.push_single_character_token_object(Dot)),
            b'-' => Ok(self.push_single_character_token_object(Minus)),
            b'+' => Ok(self.push_single_character_token_object(Plus)),
            b';' => Ok(self.push_single_character_token_object(Semicolon)),
            b'*' => Ok(self.push_single_character_token_object(Star)),

            // Two character lexeme
            b'!' => Ok(self.push_two_character_token_object(b'=', BangEqual, Bang)),
            b'=' => Ok(self.push_two_character_token_object(b'=', EqualEqual, Equal)),
            b'>' => Ok(self.push_two_character_token_object(b'=', LessEqual, Less)),
            b'<' => Ok(self.push_two_character_token_object(b'=', GreaterEqual, Greater)),

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
                    Ok(self.push_single_character_token_object(SingleLineComment))
                } else {
                    Ok(self.push_single_character_token_object(Slash))
                }
            }

            // Ignored whitespace
            b' ' | b'\r' | b'\t' => Ok(()),

            b'\n' => {
                self.line += 1;
                Ok(())
            }

            // Strings
            b'"' => self.string(),

            // Numbers
            b'0'..=b'9' => self.number(),

            // Keywords or identifiers
            b'_' | b'a'..=b'z' | b'A'..=b'Z' => self.keyword_or_identifier(),

            e => Err(ScanError::UnexpectedCharacter(e)),
        }
    }

    fn push_single_character_token_object(&mut self, next_token_type: TokenType) {
        let next_token = self.create_token_object(next_token_type, None);
        self.tokens.push(next_token)
    }

    fn push_two_character_token_object(
        &mut self,
        expected: u8,
        double: TokenType,
        single: TokenType,
    ) {
        let next_token_type = if self.match_next(expected) {
            double
        } else {
            single
        };
        let next_token = self.create_token_object(next_token_type, None);
        self.tokens.push(next_token)
    }

    fn peek(&self) -> Option<u8> {
        if self.is_at_end() {
            None
        } else {
            Some(self.source[self.current])
        }
    }

    fn peek_next(&self) -> Option<u8> {
        if self.current + 1 >= self.source.len() {
            None
        } else {
            Some(self.source[self.current + 1])
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
        literal: Option<Literal<'source>>,
    ) -> Token<'source> {
        let text = &self.source[self.start..self.current];

        Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
        }
    }

    fn string(&mut self) -> Result<(), ScanError> {
        while self.peek() != Some(b'"') && !self.is_at_end() {
            if self.peek() == Some(b'\n') {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            Err(ScanError::UnterminatedString)
        } else {
            // Closing quote
            self.advance();

            let value = &self.source[self.start + 1..self.current - 1];

            let next_token =
                self.create_token_object(TokenType::String, Some(Literal::String(value)));

            Ok(self.tokens.push(next_token))
        }
    }

    fn number(&mut self) -> Result<(), ScanError> {
        self.consume_digits();

        if self.peek() == Some(b'.') {
            if let Some(ch) = self.peek_next() {
                if ch >= b'0' && ch <= b'9' {
                    self.consume_digits();
                }
            }
        }

        let value = &self.source[self.start..self.current];

        // Ugly but we're fairly certain these bytes represent a valid number
        let parsed_f64 = from_utf8(value).unwrap().parse::<f64>().unwrap();

        let next_token =
            self.create_token_object(TokenType::Number, Some(Literal::Number(parsed_f64)));
        Ok(self.tokens.push(next_token))
    }

    fn consume_digits(&mut self) {
        while let Some(ch) = self.peek() {
            if !(ch >= b'0' && ch <= b'9') {
                break;
            }

            self.advance();
        }
    }

    fn keyword_or_identifier(&mut self) -> Result<(), ScanError> {
        while let Some(ch) = self.peek() {
            if !((ch >= b'0' && ch <= b'9') || ch.is_ascii_alphabetic()) {
                break;
            }

            self.advance();
        }

        let text = from_utf8(&self.source[self.start..self.current]).unwrap();
        let token_type = *self
            .reserved_keywords
            .get(text)
            .unwrap_or(&TokenType::Identifier);

        let next_token = self.create_token_object(token_type, None);
        Ok(self.tokens.push(next_token))
    }
}
