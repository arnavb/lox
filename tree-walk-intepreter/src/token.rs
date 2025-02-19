use std::fmt::{self, Display};

#[derive(Debug)]
pub enum TokenType {
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
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Debug)]
pub enum Literal {
    Number(i32),
    String(String),
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::String(string) => write!(f, "{}", string),
        }
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a [u8],
    pub literal: Option<Literal>,
    pub line: usize,
}
