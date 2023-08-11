use std::fs::read_to_string;
use engine::exec::{Executor, Interpreter};
use engine::syntax::{lexer::Lexer, parser::Parser};

fn main() {
    let buffer = read_to_string("tests/js/test.js").unwrap();
    let mut lexer = Lexer::new(&buffer);
    lexer.lex().unwrap();
    let tokens = lexer.tokens;
    let expr = Parser::new(tokens).parse_all().unwrap();
    let mut engine: Interpreter = Executor::new();
    let result = engine.run(&expr);
    match result {
        Ok(v) => print!("{}", v),
        Err(v) => print!("Error: {}", v),
    }
}
