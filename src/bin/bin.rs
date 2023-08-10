use std::fs::read_to_string;
use js::syntax::{lexer::Lexer, parser::Parser};

fn main() {
    let buffer = read_to_string("tests/js/defineVar.js").unwrap();
    let mut lexer = Lexer::new(&buffer);
    lexer.lex().unwrap();
    let tokens = lexer.tokens;
    match Parser::new(tokens).parse_all() {
        Ok(e) => println!("{}", e),
        Err(e) => println!("{:?}", e),
    }
}
