use std::fmt;

use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TokenKind {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Colon,

    // two character tokenss
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    PlusEqual,
    MinusEqual,

    // reserved keywords
    If,
    Else,
    For,
    While,
    Case,
    Proc,
    Ptr,
    Var,

    // types
    IntType,
    StrType,

    // literals
    Identifier(String),
    StringLit(String),
    IntLit(i32),

    NewLine,
    Eof,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

impl Token {
    pub fn of(kind: TokenKind, start: usize, end: usize, line: usize) -> Token {
        Token {
            kind,
            start,
            end,
            line,
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_val = match &*self {
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::Comma => ",",
            TokenKind::Dot => ".",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            _ => "unknown",
        };

        write!(f, "{}", str_val)
    }
}

