use std::{
    fmt::{self, Display},
    str::from_utf8,
};

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

    // TODO: Added for simplicity for now
    SingleLineComment,
}

#[derive(Debug)]
pub enum Literal<'source> {
    Number(f64),
    String(&'source [u8]),
}

impl Display for Literal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::String(string) => write!(
                f,
                "{}",
                from_utf8(string).expect("Invalid UTF-8 when formatting literal")
            ),
        }
    }
}

#[derive(Debug)]
pub struct Token<'source> {
    pub token_type: TokenType,
    pub lexeme: &'source [u8],
    pub literal: Option<Literal<'source>>,
    pub line: usize,
}
