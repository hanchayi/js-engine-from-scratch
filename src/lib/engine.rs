use gc::Gc;

use crate::{syntax::{lexer::Lexer, parser::Parser}, exec::{Executor, Interpreter}, js::value::ValueData};

pub fn run_script(script: String) -> Gc<ValueData>{
    let mut lexer = Lexer::new(&script);
    lexer.lex().unwrap();
    let tokens = lexer.tokens;
    let expr = Parser::new(tokens).parse_all().unwrap();

    let mut engine: Interpreter = Executor::new();
    let result = engine.run(&expr).unwrap();

    return result
}