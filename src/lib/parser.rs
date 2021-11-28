use crate::lexer::token::Token;

// AST
use crate::ast::*;

pub struct Parser {
    position: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            position: 0,
            tokens: tokens
        }
    }

    fn at_end(&self) -> bool {
        self.peek() == Token::Eof
    }

    fn prev(&self) -> Token {
        self.tokens[self.position - 1].clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.position].clone()
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.position += 1
        }
        self.prev()
    }

    /// Check if the current token is the token we pass in and
    /// we are not at the end of the token stream.
    fn is_type(&self, token: &Token) -> bool {
        if self.at_end() {
            return false
        }

        self.peek() == *token
    }

    /// Check if the current token matches any of the tokens passed to
    /// the function.
    fn has_match(&mut self, tokens: &[Token]) -> bool {
        for t in tokens {
            if self.is_type(t) {
                self.advance();
                return true
            }
        }
        false
    }

    /// Parsing
    fn parse_expr(&mut self) -> Expr {
        // expression ::= equality
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Expr {
        // equality ::= comparison ( ("!=" | "==") comparison)*
        let mut expr = self.parse_comparison();
        while self.has_match(&[Token::BangEqual, Token::EqualEqual]) {
            let op = self.prev();
            let rhs = self.parse_comparison();
            expr = Expr::BinOp(Box::new(expr), op, Box::new(rhs));
        }

        expr
    }

    fn parse_comparison(&mut self) -> Expr {
        // comparison ::= term ( (">" | ">=" | "<" | "<=") term)*
        let mut expr = self.parse_term();
        while self.has_match(&[Token::Greater, Token::GreaterEqual, 
                               Token::Less, Token::LessEqual]) {
            let op = self.prev();
            let rhs = self.parse_term();
            expr = Expr::BinOp(Box::new(expr), op, Box::new(rhs));
        }

        expr
    }

    fn parse_term(&mut self) -> Expr {
        // term ::= factor ( ("+" | "-") factor)*
        let mut expr = self.parse_factor();
        while self.has_match(&[Token::Plus, Token::Minus]) {
            let op = self.prev();
            let rhs = self.parse_factor();
            expr = Expr::BinOp(Box::new(expr), op, Box::new(rhs));
        }

        expr
    }

    fn parse_factor(&mut self) -> Expr {
        // factor ::= unary ( ("*" | "/") unary)*
        let mut expr = self.parse_unary();
        while self.has_match(&[Token::Star, Token::Slash]) {
            let op = self.prev();
            let rhs = self.parse_unary();
            expr = Expr::BinOp(Box::new(expr), op, Box::new(rhs));
        }

        expr
    }

    fn parse_unary(&mut self) -> Expr {
        // unary ::= ("!" | "-") unary
        //        | primary 
        if self.has_match(&[Token::Bang, Token::Minus]) {
            let op = self.prev();
            let rhs = self.parse_factor();
            return Expr::UnaryOp(op, Box::new(rhs));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Expr {
        if let Token::IntLit(n) = self.peek() {
            self.advance();
            return Expr::IntLit(n)
        }
        
        if let Token::StringLit(s) = self.peek() {
            self.advance();
            return Expr::StringLit(s)
        }

        panic!("expected an expression")
    }

    pub fn parse(&mut self) -> Expr {
        self.parse_expr()
    }
}
