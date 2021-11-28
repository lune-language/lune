use lune::lexer::lexer::Lexer;
use lune::lexer::token::Token;

use lune::parser::Parser;

fn main() {
    let mut lexer = Lexer::new(r#"2 + 2
    "#);
    let tokens: Vec<Token> = lexer.scan();
    let mut parser = Parser::new(tokens.clone());

    println!("tokens: {:?}", tokens);
    println!("parsed: {:?}", parser.parse());
}