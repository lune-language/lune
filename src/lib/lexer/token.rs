#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LParen, RParen, LBrace, RBrace,
    Comma, Dot, Plus, Minus, Star, Slash, Colon,

    // two character tokenss
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    PlusEqual, MinusEqual,

    // reserved keywords
    If, Else, For, While, Case, Proc, Ptr, Var,

    // types
    IntType, StrType,

    // literals
    Identifier(String),
    StringLit(String),
    IntLit(i32),

    NewLine, 
    Eof,
}