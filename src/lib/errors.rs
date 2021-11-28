use crate::frontend::lexer::token::Token;
use std::fmt;

#[derive(Debug)]
pub struct LexerError {
    pub position: usize,
    pub line: usize,
    pub message: String,
}

#[derive(Debug)]
pub struct ParseError {
    pub position: usize,
    pub curr_token: Token,
    pub message: String,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LexerError: error at line {}, position {}: {}",
            self.line, self.position, self.message
        )
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ParseError: error at position {}, token {:?}: {}",
            self.position, self.curr_token, self.message
        )
    }
}
