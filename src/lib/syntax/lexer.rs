use std::str::Chars;

use super::ast::{token::{Token, TokenData}, punc::Punctuator};

/// js词法分析器
pub struct Lexer<'a> {
    pub tokens: Vec<Token>,
    pub line_number: u64,
    pub column_number: u64,
    pub buffer: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(buffer: &'a str) -> Lexer<'a> {
        Lexer {
            tokens: Vec::new(),
            buffer: buffer.chars(),
            line_number: 1,
            column_number: 0,
        }
    }

    // 添加一个标点符号Token
    pub fn push_punctuator(&mut self, punc: Punctuator) {
        self.push_token(TokenData::TPunctuator(punc))
    }

    // pub fn lex_str(script: String) -> Vec<Token>{
    //     let mut lexer = Lexer::new(script);
    //     lexer.tokens
    // }

    /// 添加一个Token
    pub fn push_token(&mut self, token_data: TokenData) {
        self.tokens.push(Token::new(token_data, self.line_number, self.column_number))
    }

    pub fn lex(&mut self) {
        loop {
            let ch = match self.next() {
                Some(char) => {
                    println!("{}", char)
                },
                None => {
                    break;
                },
            };
        }
    }

    fn next(&mut self) -> Option<char>{
        self.buffer.next()
    }
}