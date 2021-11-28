use lune::frontend::lexer::lexer::Lexer;
use lune::frontend::lexer::token::Token;

use lune::frontend::parser::Parser;
use lune::backend::ast_printer::ASTPrinter;

fn main() {
    let mut lexer = Lexer::new(r#"
    var x : str = "hello world"
    var y : int = \
        5 + 4 \
            - 2
    "#);
    let tokens: Vec<Token> = lexer.scan();
    let mut parser = Parser::new(tokens.clone());

    println!("tokens: {:?}", tokens);

    let mut printer = ASTPrinter {};

    match parser.parse() {
        Ok(result) => printer.print(result),
        Err(err) => eprintln!("{}", err)
    }
}