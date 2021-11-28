use crate::errors::ParseError;

use crate::frontend::lexer::token::Token;
use crate::types::Type;

// AST
use crate::backend::ast::*;

pub struct Parser {
    position: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            position: 0,
            tokens: tokens,
        }
    }

    fn at_end(&self) -> bool {
        self.peek() == Token::Eof
    }

    fn prev(&self) -> Token {
        self.tokens.get(self.position - 1).unwrap().clone()
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.position).unwrap().clone()
    }

    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.position += 1
        }
        self.prev()
    }

    /// Check if the current token is the token we pass in, and consume it
    fn consume(&mut self, token: &Token) -> Option<Token> {
        if self.is_type(token) {
            Some(self.advance())
        } else {
            None
        }
    }

    /// Check if the current token is the token we pass in, and consume it
    fn consume_if(&mut self, f: impl FnOnce(&Token) -> bool) -> Option<Token> {
        let token = self.tokens.get(self.position).unwrap();
        if f(token) {
            Some(self.advance())
        } else {
            None
        }
    }

    /// Check if the current token is the token we pass in and
    /// we are not at the end of the token stream.
    fn is_type(&self, token: &Token) -> bool {
        if self.at_end() {
            return false;
        }

        self.peek() == *token
    }

    /// Check if the current token matches any of the tokens passed to
    /// the function.
    fn has_match(&mut self, tokens: &[Token]) -> bool {
        for t in tokens {
            if self.is_type(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Parsing
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        // expression ::= equality
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        // equality ::= comparison ( ("!=" | "==") comparison)*
        let mut expr = self.parse_comparison()?;
        while self.has_match(&[Token::BangEqual, Token::EqualEqual]) {
            let op = self.prev();
            let rhs = self.parse_comparison()?;
            expr = Expr::BinOp(Box::new(expr), op, Box::new(rhs));
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        // comparison ::= term ( (">" | ">=" | "<" | "<=") term)*
        let mut expr = self.parse_term()?;
        while self.has_match(&[
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual,
        ]) {
            let op = self.prev();
            let rhs = self.parse_term()?;
            expr = Expr::BinOp(Box::new(expr), op, Box::new(rhs));
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        // term ::= factor ( ("+" | "-") factor)*
        let mut expr = self.parse_factor()?;
        while self.has_match(&[Token::Plus, Token::Minus]) {
            let op = self.prev();
            let rhs = self.parse_factor()?;
            expr = Expr::BinOp(Box::new(expr), op, Box::new(rhs));
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        // factor ::= unary ( ("*" | "/") unary)*
        let mut expr = self.parse_unary()?;
        while self.has_match(&[Token::Star, Token::Slash]) {
            let op = self.prev();
            let rhs = self.parse_unary()?;
            expr = Expr::BinOp(Box::new(expr), op, Box::new(rhs));
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        // unary ::= ("!" | "-") unary
        //        | primary
        if self.has_match(&[Token::Bang, Token::Minus]) {
            let op = self.prev();
            let rhs = self.parse_factor()?;
            return Ok(Expr::UnaryOp(op, Box::new(rhs)));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        if let Token::IntLit(n) = self.peek() {
            self.advance();
            return Ok(Expr::IntLit(n));
        }
        if let Token::StringLit(s) = self.peek() {
            self.advance();
            return Ok(Expr::StringLit(s));
        }

        Err(ParseError {
            curr_token: self.peek(),
            position: self.position,
            message: "invalid expression".into(),
        })
    }

    // Statements
    fn parse_var(&mut self) -> Result<Stmt, ParseError> {
        let mut name = String::from("");
        let type_: Type;
        let mut value: Option<Expr> = None;

        if let Some(Token::Identifier(ident)) =
            self.consume_if(|token| matches!(token, Token::Identifier(_)))
        {
            name = ident.to_owned();
        }

        match self.consume(&Token::Colon) {
            Some(_) => {}
            None => panic!("error: expected a colon"),
        }

        match self.peek() {
            Token::IntType => type_ = Type::Int,
            Token::StrType => type_ = Type::String,
            _ => panic!("error: invalid type"),
        }

        self.advance();

        if matches!(self.peek(), Token::Equal) {
            self.advance();
            value = Some(self.parse_expr()?);
        }

        Ok(Stmt::VarDeclaration(
            Name { value: name },
            type_,
            value.expect("expected value"),
        ))
    }

    pub fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        match self.peek() {
            Token::Var => {
                self.advance();
                return self.parse_var();
            }
            _ => Err(ParseError {
                curr_token: self.peek(),
                position: self.position,
                message: "invalid statement type".into(),
            }),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts: Vec<Stmt> = Vec::new();

        loop {
            stmts.push(self.parse_stmt()?);
            if self.at_end() {
                // We've reached the end, break out
                break;
            }
        }
        Ok(stmts)
    }
}
