use std::fs::read_to_string;
use js::syntax::lexer::Lexer;

fn main() {
    let buffer = read_to_string("tests/helloworld.js").unwrap();
    let lexer = Lexer::new(buffer);
    lexer.lex();
}
