use lune::frontend::lexer::lexer::Lexer;
use lune::frontend::lexer::token::Token;

use lune::frontend::parser::Parser;
use lune::backend::ast_dumper::ASTDumper;
//use lune::backend::ast_printer::ASTPrinter;

fn main() {
    let mut lexer = Lexer::new(
        r#"var aNumber:int=0x2000
           var aString:str="hello world""#,
    );
    let tokens: Vec<Token> = lexer.scan();
    let mut parser = Parser::new(tokens.clone());

    println!("tokens: {:?}", tokens);
    //println!("parsed: {:?}", parser.parse());

    let dumper = ASTDumper {};
    dumper.dump_ast(parser.parse().unwrap());

    /*
    let mut printer = ASTPrinter {};

    match parser.parse() {
        Ok(result) => printer.print(result),
        Err(err) => eprintln!("{}", err)
    }*/
}
