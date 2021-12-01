/// AST nodes
use std::fmt;

use crate::frontend::lexer::token::Token;
use crate::types::Type;

/// Type alias for AST
pub type AST = Vec<Stmt>;

#[derive(Debug, PartialEq)]
pub enum Expr {
    IntLit(i32),
    StringLit(String),

    /// Unary operator eg -1
    UnaryOp(Token, Box<Expr>),

    /// Binary operator eg 2 + 4
    BinOp(Box<Expr>, Token, Box<Expr>)
}

#[derive(Debug, PartialEq)]
pub struct Name {
    pub value: String
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),

    /// Variable declaration
    VarDeclaration(Name, Type, Expr),

    //IfStatement(Comparison, )

    /// Assignment
    Assignment(Name, Expr),
}

///
/// Visitor trait that structs can inherit from to walk the AST
///
pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn visit_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_name(&mut self, name: &Name) -> T;
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result = match &*self {
            Expr::IntLit(n) => write!(f, "{}", n),
            Expr::StringLit(s) => write!(f, "\"{}\"", s),
            Expr::BinOp(lhs, op, rhs) => write!(f, "{}{}{}", lhs, op.kind, rhs),
            Expr::UnaryOp(op, rhs) => write!(f, "{}{}", op.kind, rhs),
        };

        result
    }
}