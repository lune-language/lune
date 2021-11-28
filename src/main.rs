use lune::lexer::lexer::Lexer;
use lune::lexer::token::Token;

fn main() {
    let mut lexer = Lexer::new(r#"
    # single comment
    #
    # multiline
    # comment
    # blah
    #
    var x : int = 0
    var y : str = "hello world!"
    var z : ptr = \
        (5 + \
            1)

    "#);
    let tokens: Vec<Token> = lexer.scan();

    println!("tokens: {:?}", tokens);
}