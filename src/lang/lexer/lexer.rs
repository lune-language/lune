use lazy_static::lazy_static;
use std::collections::HashMap;
use std::vec;

use crate::lexer::token::Token;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Token> = HashMap::from([
        ("if", Token::If),
        ("else", Token::Else),
        ("for", Token::For),
        ("while", Token::While),
        ("case", Token::Case),
        ("proc", Token::Proc),
        ("ptr", Token::Ptr),
        ("var", Token::Var),

        ("int", Token::IntType),
        ("str", Token::StrType)
    ]);
}

pub struct Lexer<'a> {
    // start and current position in the stream
    position: usize,
    line: usize,
    src: &'a [u8],
    pub tokens: Vec<Token>,
}

impl Lexer<'_> {
    pub fn new(src: &str) -> Lexer {
        Lexer {
            position: 0,
            line: 1,
            src: src.as_bytes(),
            tokens: vec![],
        }
    }

    fn peek(&self) -> char {
        if self.position >= self.src.len() {
            return '\0';
        }

        self.src[self.position] as char
    }

    fn prev(&self) -> char {
        self.src[self.position - 1] as char
    }

    fn next(&self, ahead: usize) -> char {
        if self.position + ahead >= self.src.len() {
            return '\0';
        }

        self.src[self.position + ahead] as char
    }

    fn advance(&mut self) {
        self.position += 1
    }

    fn push(&mut self, length: usize, tok: Token) {
        self.tokens.push(tok);
        self.position += length
    }

    fn expect(&self, c: char, msg: &str) {
        if self.peek() != c {
            panic!("{}", msg);
        }
    }

    fn scan_int(&mut self) {
        let mut lexed = String::new();

        while self.peek().is_ascii_digit() {
            lexed.push(self.peek());
            self.advance();
        }

        let value = lexed.parse::<i32>()
                         .expect("error: invalid integer literal");

        self.push(0, Token::IntLit(value));
    }

    fn scan_string(&mut self) {
        // "hello world"
        let mut lexed = String::new();
        self.advance();

        while self.peek() != '"' && self.position < self.src.len() {
            match (self.peek(), self.next(1)) {
                ('\\', 'n') => lexed.push('\n'),
                ('\\', 't') => lexed.push('\t'),
                ('\\', 'r') => lexed.push('\r'),
                ('\\', '\\') => lexed.push('\\'),
                (c, _) => lexed.push(c),
            }
            self.advance();
        }

        self.expect('"', "error: unexpected end of string");
        self.push(2, Token::StringLit(lexed));
    }

    fn scan_ident(&mut self) {
        let mut lexed = String::new();

        while self.peek().is_ascii_alphabetic() {
            lexed.push(self.peek());
            self.advance();
        }

        //println!("lexed: {}", lexed);
        match KEYWORDS.get(lexed.as_str()) {
            Some(v) => self.push(0, v.clone()),
            None => self.push(0, Token::Identifier(lexed))
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        while self.position < self.src.len() {
            match (self.peek(), self.next(1)) {
                ('(', _) => self.push(1, Token::LParen),
                (')', _) => self.push(1, Token::RParen),
                ('{', _) => self.push(1, Token::LBrace),
                ('}', _) => self.push(1, Token::RBrace),
                (',', _) => self.push(1, Token::Comma),
                ('.', _) => self.push(1, Token::Dot),
                ('+', '=') => self.push(2, Token::PlusEqual),
                ('+', _) => self.push(1, Token::Plus),
                ('-', '=') => self.push(2, Token::MinusEqual),
                ('-', _) => self.push(1, Token::Minus),
                ('*', _) => self.push(1, Token::Star),
                ('/', _) => self.push(1, Token::Slash),

                (':', _) => self.push(1, Token::Colon),
                ('!', '=') => self.push(2, Token::BangEqual),
                ('=', '=') => self.push(2, Token::EqualEqual),
                ('<', '=') => self.push(2, Token::LessEqual),
                ('>', '=') => self.push(2, Token::GreaterEqual),

                ('=', _) => self.push(1, Token::Equal),
                ('<', _) => self.push(1, Token::Less),
                ('>', _) => self.push(1, Token::Greater),

                ('"', _) => self.scan_string(),
                (d, _) if d.is_digit(10) => self.scan_int(),
                (w, _) if w.is_whitespace() => self.advance(),
                (id, _) if id.is_ascii_alphabetic() => self.scan_ident(),

                // Comments
                ('#', _) => {
                    while self.peek() != '\n' && self.position < self.src.len() {
                        self.advance();
                    }
                    if self.peek() == '\n' {
                        self.advance();
                    }

                    self.line += 1;
                }

                // Newlines and continuations
                ('\n', _) => {
                    self.push(1, Token::NewLine);
                    self.line += 1;
                }
                ('\\', '\n') => {
                    self.line += 1;
                    self.advance();
                }

                (_, _) => self.advance()
            }
        }
        
        self.push(1, Token::Eof);
        self.tokens.clone()
    }
}