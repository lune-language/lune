pub mod lexer;
pub mod parser;
pub mod ast;
pub mod errors;

pub mod types {
    /// Enum of supported types
    #[derive(Debug, PartialEq)]
    pub enum Type {
        // TODO: add pointer for interop with C
        Int,    // i32
        String,
        Bool,
    }
}