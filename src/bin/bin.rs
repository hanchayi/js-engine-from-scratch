use std::fs::read_to_string;
use js::syntax::lexer::Lexer;

fn main() {
    let buffer = read_to_string("tests/helloworld.js").unwrap();
    let mut lexer = Lexer::new(&buffer);
    lexer.lex().expect("finished")
}
