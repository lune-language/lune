/// AST nodes

use crate::lexer::token::Token;
use crate::types::Type;

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    VarDeclaration(Name, Type, Expr),
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