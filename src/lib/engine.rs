use crate::{syntax::{lexer::Lexer, parser::Parser}, exec::{Executor, Interpreter}};

pub fn run_script(script: String) {
    let mut lexer = Lexer::new(&script);
    lexer.lex().unwrap();
    let tokens = lexer.tokens;
    let expr = Parser::new(tokens).parse_all().unwrap();
    println!("{}", expr);

    let mut engine: Interpreter = Executor::new();
    let result = engine.run(&expr);
    match result {
        Ok(v) => print!("{}", v),
        Err(v) => print!("Error: {}", v),
    }
}