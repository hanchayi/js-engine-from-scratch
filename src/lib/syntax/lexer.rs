use super::ast::token::Token;

pub struct Lexer {
    pub tokens: Vec<Token>,
    pub line_number: u64,
    pub column_number: u64,
    pub buffer: String,
}

impl Lexer {
    pub fn new(buffer: String) -> Lexer {
        Lexer {
            tokens: Vec::new(),
            buffer: buffer,
            line_number: 1,
            column_number: 0,
        }
    }

    pub fn lex(& self) {
        println!("{}", self.buffer)
    }
}