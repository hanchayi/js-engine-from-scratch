use super::ast::token::{Token, TokenData};

/// js词法分析器
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

    /// 添加一个Token
    pub fn push_token(&mut self, token_data: TokenData) {
        self.tokens.push(Token::new(token_data, self.line_number, self.column_number))
    }

    pub fn lex(& self) {
        println!("{}", self.buffer)
    }
}