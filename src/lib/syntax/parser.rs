use super::ast::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: u64,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens: tokens, pos: 0 }
    }
}