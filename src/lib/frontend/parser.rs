use crate::errors::ParseError;

use crate::frontend::lexer::token::{Token, TokenKind};
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
            tokens,
        }
    }

    fn at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }

    /// Return the previous token
    fn prev(&self) -> Token {
        self.tokens.get(self.position - 1).unwrap().clone()
    }

    /// Peek ahead and return the token 
    fn peek(&self) -> Token {
        self.tokens.get(self.position).unwrap().clone()
    }

    /// Advance the position and return the previous token
    fn advance(&mut self) -> Token {
        if !self.at_end() {
            self.position += 1
        }
        self.prev()
    }

    /// Consume the current token and return the next token
    fn consume(&mut self, kind: TokenKind) -> Option<Token> {
        if !self.at_end() && self.peek().kind == kind {
            Some(self.advance())
        } else {
            None
        }
    }

    /// Return the matched token if there is one, otherwise return None
    /// If we are at the end of the stream, just return None.
    fn matches(&mut self, kinds: &[TokenKind]) -> Option<Token> {
        for kind in kinds {
            if !self.at_end() && self.peek().kind == *kind {
                //self.advance();
                return Some(self.peek());
            }
        }
        None
    }

    /// Parsing
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        // expression ::= equality
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expr, ParseError> {
        // equality ::= comparison ( ("!=" | "==") comparison)*
        let mut expr = self.parse_comparison()?;
        while let Some(op) = self.matches(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let rhs = self.parse_comparison()?;
            expr = Expr::BinOp(Box::new(expr), op, Box::new(rhs));
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, ParseError> {
        // comparison ::= term ( (">" | ">=" | "<" | "<=") term)*
        let mut expr = self.parse_term()?;
        while let Some(op) = self.matches(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let rhs = self.parse_term()?;
            expr = Expr::BinOp(Box::new(expr), op, Box::new(rhs));
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        // term ::= factor ( ("+" | "-") factor)*
        let mut expr = self.parse_factor()?;
        while let Some(op) = self.matches(&[TokenKind::Plus, TokenKind::Minus]) {
            let rhs = self.parse_factor()?;
            expr = Expr::BinOp(Box::new(expr), op, Box::new(rhs));
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        // factor ::= unary ( ("*" | "/") unary)*
        let mut expr = self.parse_unary()?;
        while let Some(op) = self.matches(&[TokenKind::Star, TokenKind::Slash]) {
            let rhs = self.parse_unary()?;
            expr = Expr::BinOp(Box::new(expr), op, Box::new(rhs));
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        // unary ::= ("!" | "-") unary
        //        | primary
        if let Some(op) = self.matches(&[TokenKind::Bang, TokenKind::Minus]) {
            let rhs = self.parse_factor()?;
            return Ok(Expr::UnaryOp(op, Box::new(rhs)));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        if let TokenKind::IntLit(n) = self.peek().kind {
            self.advance();
            return Ok(Expr::IntLit(n));
        }
        if let TokenKind::StringLit(s) = self.peek().kind {
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

        // Consume the identifier
        match self.advance().kind {
            TokenKind::Identifier(ident) => name = ident,
            _ => {},
        }

        match self.consume(TokenKind::Colon) {
            Some(_) => {}
            None => panic!("error: expected a colon"),
        }

        match self.peek().kind {
            TokenKind::IntType => type_ = Type::Int,
            TokenKind::StrType => type_ = Type::String,
            _ => panic!("error: invalid type"),
        }

        self.advance();

        if matches!(self.peek().kind, TokenKind::Equal) {
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
        match self.peek().kind {
            TokenKind::Var => {
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
