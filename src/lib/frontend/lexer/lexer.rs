use lazy_static::lazy_static;
use std::collections::HashMap;
use std::vec;

//use crate::errors::LexerError;
use super::token::{Token, TokenKind};

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenKind> = HashMap::from([
        ("if", TokenKind::If),
        ("else", TokenKind::Else),
        ("for", TokenKind::For),
        ("while", TokenKind::While),
        ("case", TokenKind::Case),
        ("proc", TokenKind::Proc),
        ("ptr", TokenKind::Ptr),
        ("var", TokenKind::Var),
        ("int", TokenKind::IntType),
        ("str", TokenKind::StrType)
    ]);
}

pub struct Lexer<'lex> {
    // start and current position in the stream
    start: usize,
    current: usize,
    line: usize,

    source: &'lex str,
    pub tokens: Vec<Token>,
}

impl<'lex> Lexer<'lex> {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            start: 0,
            current: 0,
            line: 0,

            source,
            tokens: vec![],
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Peek ahead and return the character
    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    /// Peek ahead by `ahead` characters and return the character
    fn peek_next(&self, ahead: usize) -> char {
        self.source
            .chars()
            .nth(self.current + ahead)
            .unwrap_or('\0')
    }

    /// Advance the lexer position
    fn advance(&mut self) {
        self.current += 1;
    }

    /// Push a token of `kind` and advance the lexer by `length`
    fn push(&mut self, length: usize, kind: TokenKind) {
        self.tokens
            .push(Token::of(kind, self.start, self.current, self.line));
        self.current += length;
    }

    fn expect(&self, c: char, msg: &str) {
        if self.peek() != c {}
    }

    /// Scan hexadecimal format integers
    fn scan_hex_int(&mut self) {
        let hex_chars = "0123456789ABCDEFabcdef";

        // Skip over 0x
        self.advance();
        self.advance();

        while hex_chars.contains(self.peek()) {
            self.advance();
        }

        let lexed: String = self.source[self.start + 2..self.current].into();
        self.push(
            0,
            TokenKind::IntLit(i32::from_str_radix(&lexed, 16).expect("failed to encode")),
        );
    }

    /// Scan binary format integers
    fn scan_bin_int(&mut self) {
        let bin_chars = "01";

        // Skip over 0b
        self.advance();
        self.advance();

        while bin_chars.contains(self.peek()) {
            self.advance();
        }

        let lexed: String = self.source[self.start + 2..self.current].into();
        self.push(
            0,
            TokenKind::IntLit(i32::from_str_radix(&lexed, 2).expect("failed to encode")),
        );
    }

    fn scan_int(&mut self) {
        let mut has_radix = false;

        if self.peek() == '0' {
            self.advance();
        }

        // NOTE: should we support underscores in integer literals?
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Match on the radix
        match self.peek() {
            'b' => {
                has_radix = true;
                self.scan_bin_int()
            }
            'x' => {
                has_radix = true;
                self.scan_hex_int()
            }
            _ => {}
        }

        if !has_radix {
            // Parse as decimal
            let lexed: String = self.source[self.start..self.current].into();
            self.push(
                0,
                TokenKind::IntLit(i32::from_str_radix(&lexed, 10).expect("failed to encode")),
            );
        }
    }

    fn scan_ident(&mut self) {
        while self.peek().is_ascii_alphabetic() || self.peek() == '_' {
            self.advance();
        }

        let lexed: String = self.source[self.start..self.current].into();

        match KEYWORDS.get(&*lexed) {
            Some(k) => self.push(0, k.clone()),
            None => self.push(0, TokenKind::Identifier(lexed)),
        }
    }

    fn scan_string(&mut self) {
        self.advance();
        while self.peek() != '"' && !self.at_end() {
            self.advance();
        }

        let lexed: String = self.source[self.start+1..self.current].into();

        self.expect('"', "unexpected end of string");
        self.push(2, TokenKind::StringLit(lexed));
    }

    pub fn scan(&mut self) -> Vec<Token> {
        while !self.at_end() {
            self.start = self.current;

            match (self.peek(), self.peek_next(1)) {
                ('(', _) => self.push(1, TokenKind::LParen),
                (')', _) => self.push(1, TokenKind::RParen),
                ('{', _) => self.push(1, TokenKind::LBrace),
                ('}', _) => self.push(1, TokenKind::RBrace),
                (',', _) => self.push(1, TokenKind::Comma),
                ('.', _) => self.push(1, TokenKind::Dot),
                ('+', '=') => self.push(2, TokenKind::PlusEqual),
                ('+', _) => self.push(1, TokenKind::Plus),
                ('-', '=') => self.push(2, TokenKind::MinusEqual),
                ('-', _) => self.push(1, TokenKind::Minus),
                ('*', _) => self.push(1, TokenKind::Star),
                ('/', _) => self.push(1, TokenKind::Slash),

                (':', _) => self.push(1, TokenKind::Colon),
                ('!', '=') => self.push(2, TokenKind::BangEqual),
                ('=', '=') => self.push(2, TokenKind::EqualEqual),
                ('<', '=') => self.push(2, TokenKind::LessEqual),
                ('>', '=') => self.push(2, TokenKind::GreaterEqual),

                ('=', _) => self.push(1, TokenKind::Equal),
                ('<', _) => self.push(1, TokenKind::Less),
                ('>', _) => self.push(1, TokenKind::Greater),

                ('"', _) => self.scan_string(),
                (w, _) if w.is_whitespace() => self.advance(),
                (digit, _) if digit.is_digit(10) => self.scan_int(),
                (ident, _) if ident.is_ascii_alphabetic() => self.scan_ident(),

                // Comments
                ('#', _) => {
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }

                    if self.peek() == '\n' {
                        self.advance();
                    }

                    self.line += 1;
                }

                // Newlines
                ('\n', _) => {
                    self.push(1, TokenKind::NewLine);
                    self.line += 1;
                }

                ('\\', '\n') => {
                    // Continuation, just advance
                    self.advance();
                    self.line += 1;
                }
                (_, _) => self.advance(),
            }
        }
        self.push(1, TokenKind::Eof);
        self.tokens.clone()
    }
}
