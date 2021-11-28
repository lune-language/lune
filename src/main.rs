use lune::lexer::lexer::Lexer;
use lune::lexer::token::Token;

use lune::parser::Parser;

fn main() {
    let mut lexer = Lexer::new(r#"
    var x : str = "hello world"
    var y : int = \
        3 \
        + 4
    "#);
    let tokens: Vec<Token> = lexer.scan();
    let mut parser = Parser::new(tokens.clone());

    println!("tokens: {:?}", tokens);

    match parser.parse() {
        Ok(result) => println!("parsed: {:?}", result),
        Err(err) => eprintln!("{}", err)
    }
}