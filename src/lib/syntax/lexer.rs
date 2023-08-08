use std::{iter::Peekable, fmt::Display, error, char::from_u32};
use std::str::FromStr;
use std::str::Chars;
use super::ast::{token::{Token, TokenData}, punc::Punctuator};

#[derive(Debug, Clone)]
pub struct  LexerError {
    details: String,
}

impl LexerError {
    pub fn new (msg: &str) -> LexerError {
        LexerError { details: msg.to_string() }
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl error::Error for LexerError {
    fn description(&self) -> &str {
        &self.details
    }
    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

/// js词法分析器
pub struct Lexer<'a> {
    pub tokens: Vec<Token>,
    pub line_number: u64,
    pub column_number: u64,
    pub buffer: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(buffer: &'a str) -> Lexer<'a> {
        Lexer {
            tokens: Vec::new(),
            buffer: buffer.chars().peekable(),
            line_number: 1,
            column_number: 0,
        }
    }

    // 添加一个标点符号Token
    pub fn push_punc(&mut self, punc: Punctuator) {
        self.push_token(TokenData::Punctuator(punc))
    }

    // pub fn lex_str(script: String) -> Vec<Token>{
    //     let mut lexer = Lexer::new(script);
    //     lexer.tokens
    // }

    /// 添加一个Token
    pub fn push_token(&mut self, token_data: TokenData) {
        self.tokens.push(Token::new(token_data, self.line_number, self.column_number))
    }

    pub fn lex(&mut self) -> Result<(), LexerError> {
        loop {
            let ch = match self.next() {
                Ok(ch) => ch,
                Err(lexer_error) => return Err(lexer_error)
            };

            match ch {
                // 字符串
                '"' | '\'' => {
                    let mut buf = String::new();
                    loop {
                        match self.next()? {
                            '\'' if ch == '\'' => {
                                break;
                            }
                            '"' if ch == '"' => {
                                break;
                            }
                            '\\' => {
                                let escape = self.next()?;
                                if escape != '\n' {
                                    let escaped_ch = match escape {
                                        'n' => '\n',
                                        'r' => '\r',
                                        't' => '\t',
                                        'b' => '\x08',
                                        'f' => '\x0c',
                                        '0' => '\0',
                                        'x' => {
                                            let mut nums = String::with_capacity(2);
                                            for _ in 0u8..2 {
                                                nums.push(self.next()?);
                                            }
                                            self.column_number += 2;
                                            let as_num = match u64::from_str_radix(&nums, 16) {
                                                Ok(v) => v,
                                                Err(_) => 0,
                                            };
                                            match from_u32(as_num as u32) {
                                                Some(v) => v,
                                                None => panic!(
                                                    "{}:{}: {} is not a valid unicode scalar value",
                                                    self.line_number, self.column_number, as_num
                                                ),
                                            }
                                        }
                                        'u' => {
                                            let mut nums = String::new();
                                            for _ in 0u8..4 {
                                                nums.push(self.next()?);
                                            }
                                            self.column_number += 4;
                                            let as_num = match u64::from_str_radix(&nums, 16) {
                                                Ok(v) => v,
                                                Err(_) => 0,
                                            };
                                            match from_u32(as_num as u32) {
                                                Some(v) => v,
                                                None => panic!(
                                                    "{}:{}: {} is not a valid unicode scalar value",
                                                    self.line_number, self.column_number, as_num
                                                ),
                                            }
                                        }
                                        '\'' | '"' => escape,
                                        _ => panic!(
                                            "{}:{}: Invalid escape `{}`",
                                            self.line_number, self.column_number, ch
                                        ),
                                    };
                                    buf.push(escaped_ch);
                                }
                            }
                            ch => buf.push(ch),
                        }
                    }
                    self.push_token(TokenData::StringLiteral(buf))
                },
                '0' => {
                    let mut buf = String::new();
                    let num = if self.next_is('x')? {
                        loop {
                            let ch = self.preview_next()?;
                            match ch {
                                ch if ch.is_digit(16) => {
                                    buf.push(self.next()?);
                                }
                                _ => break,
                            }
                        }
                        u64::from_str_radix(&buf, 16).unwrap()
                    } else {
                        loop {
                            let ch = self.preview_next()?;
                            match ch {
                                ch if ch.is_digit(8) => {
                                    buf.push(ch);
                                    self.next()?;
                                }
                                '8' | '9' | '.' => {
                                    buf.push(ch);
                                    self.next()?;
                                }
                                _ => break,
                            }
                        }
                        // if gone_decimal {
                        //     from_str(&buf)
                        // } else {
                        //     u64::from_str_radix(&buf, 8)
                        // }
                        u64::from_str_radix(&buf, 8).unwrap()

                    };
                    self.push_token(TokenData::NumericLiteral(num as f64))
                },
                _ if ch.is_digit(10) => {
                    let mut buf = ch.to_string();
                    loop {
                        let ch = self.preview_next()?;
                        match ch {
                            '.' => {
                                buf.push(self.next()?);
                            }
                            _ if ch.is_digit(10) => {
                                buf.push(self.next()?);
                            }
                            _ => break,
                        }
                    }
                    // TODO make this a bit more safe -------------------------------VVVV
                    self.push_token(TokenData::NumericLiteral(f64::from_str(&buf).unwrap()))
                }
                _ if ch.is_alphabetic() || ch == '$' || ch == '_' => {
                    let mut buf = ch.to_string();
                    loop {
                        let ch = self.preview_next()?;
                        match ch {
                            _ if ch.is_alphabetic() || ch.is_digit(10) || ch == '_' => {
                                buf.push(self.next()?);
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                    // Match won't compare &String to &str so i need to convert it :(
                    let buf_compare: &str = &buf;
                    self.push_token(match buf_compare {
                        "true" => TokenData::BooleanLiteral(true),
                        "false" => TokenData::BooleanLiteral(false),
                        "null" => TokenData::NullLiteral,
                        slice => match FromStr::from_str(slice) {
                            Ok(keyword) => TokenData::Keyword(keyword),
                            Err(_) => TokenData::Identifier(buf.clone()),
                        },
                    });
                }
                ';' => self.push_punc(Punctuator::Semicolon),
                ':' => self.push_punc(Punctuator::Colon),
                '.' => self.push_punc(Punctuator::Dot),
                '(' => self.push_punc(Punctuator::OpenParen),
                ')' => self.push_punc(Punctuator::CloseParen),
                ',' => self.push_punc(Punctuator::Comma),
                '{' => self.push_punc(Punctuator::OpenBlock),
                '}' => self.push_punc(Punctuator::CloseBlock),
                '[' => self.push_punc(Punctuator::OpenBracket),
                ']' => self.push_punc(Punctuator::CloseBracket),
                '?' => self.push_punc(Punctuator::Question),
                '/' => {
                    let token = match self.preview_next()? {
                        // Matched comment
                        '/' => {
                            let comment = self.read_line()?;
                            TokenData::Comment(comment)
                        }
                        '*' => {
                            let mut buf = String::new();
                            loop {
                                match self.next()? {
                                    '*' => {
                                        if self.next_is('/')? {
                                            break;
                                        } else {
                                            buf.push('*')
                                        }
                                    }
                                    ch => buf.push(ch),
                                }
                            }
                            TokenData::Comment(buf)
                        }
                        '=' => TokenData::Punctuator(Punctuator::AssignDiv),
                        _ => TokenData::Punctuator(Punctuator::Div),
                    };
                    self.push_token(token)
                }
                ch => panic!(
                    "{}:{}: Unexpected '{}'",
                    self.line_number, self.column_number, ch
                ),
            }
        }
    }

    fn next(&mut self) -> Result<char, LexerError>{
        self.buffer.next().ok_or(LexerError::new("next failed"))
    }

    fn preview_next(&mut self) -> Result<char, LexerError> {
        match self.buffer.peek() {
            Some(v) => Ok(*v),
            None => Err(LexerError::new("uidhi")),
        }
    }

    /// 一直读到结尾
    fn read_line(&mut self) -> Result<String, LexerError> {
        let mut buf = String::new();
        loop {
            let ch = self.next()?;
            match ch {
                _ if ch.is_ascii_control() => {
                    break;
                }
                _ => {
                    buf.push(ch);
                }
            }
        }

        Ok(buf)
    }

    fn next_is(&mut self, peek: char) -> Result<bool, LexerError> {
        let result = self.preview_next()? == peek;
        if result {
            self.buffer.next();
        }
        return Ok(result)
    }
}