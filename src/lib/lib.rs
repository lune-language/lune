// Frontend stuff like lexing and parsing
pub mod frontend;

// Backend IR and code gen
pub mod backend;

// Errors
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