use std::{iter::Peekable, fmt::Display, error, char::from_u32};
use std::str::FromStr;
use std::str::Chars;
use crate::syntax::ast::punc::Punctuator;
use crate::syntax::ast::token::{Token, TokenData};

#[allow(unused)]
macro_rules! vop {
    ($this:ident, $assign_op:expr, $op:expr) => ({
        let preview = $this.preview_next()?;
        match preview {
            '=' => {
                $this.next()?;
                $assign_op
            }
            _ => $op,
        }
    });
    ($this:ident, $assign_op:expr, $op:expr, {$($case:pat => $block:expr), +}) => ({
        let preview = $this.preview_next()?;
        match preview {
            '=' => {
                $this.next()?;
                $assign_op
            },
            $($case => $block)+,
            _ => $op
        }
    });
    ($this:ident, $op:expr, {$($case:pat => $block:expr),+}) => {
        let preview = $this.preview_next()?;
        match preview {
            $($case => $block) +,
            _ => $op
        }
    }
}

macro_rules! op {
    ($this:ident, $assign_op:expr, $op:expr) => ({
        let punc = vop!($this, $assign_op, $op);
        $this.push_punc(punc);
    });
    ($this:ident, $assign_op:expr, $op:expr, {$($case:pat => $block:expr),+}) => ({
        let punc = vop!($this, $assign_op, $op, {$($case => $block),+});
        $this.push_punc(punc);
    });
    ($this:ident, $op:expr, {$($case:pat => $block:expr),+}) => ({
        let punc = vop!($this, $op, {$($case => $block),+});
        $this.push_punc();
    });
}

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
            match self.preview_next() {
                Ok(_) => (),
                Err(e) => {
                    if e.details == "finished" {
                        return Ok(())
                    } else {
                        return Err(e)
                    }
                },
            }

            self.column_number += 1;
            let ch = self.next()?;

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
                    let str_length = buf.len() as u64;
                    self.push_token(TokenData::StringLiteral(buf));
                    self.column_number += str_length + 1;
                },
                // 匹配16进制数字
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
                        let mut gone_decimal = false;
                        loop {
                            let ch = self.preview_next()?;
                            match ch {
                                ch if ch.is_digit(8) => {
                                    buf.push(ch);
                                    self.next()?;
                                }
                                '8' | '9' | '.' => {
                                    gone_decimal = true;
                                    buf.push(ch);
                                    self.next()?;
                                }
                                _ => break,
                            }
                        }
                        if gone_decimal {
                            u64::from_str(&buf).unwrap()
                        } else {
                            if buf.is_empty() {
                                0
                            } else {
                                u64::from_str_radix(&buf, 8).unwrap()
                            }
                        }

                    };
                    self.push_token(TokenData::NumericLiteral(num as f64))
                },
                // 匹配数字字面量
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
                // 匹配字面量和关键字和标志符
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
                    self.column_number += (buf_compare.len() - 1) as u64;
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
                        // //注释
                        '/' => {
                            let comment = self.read_line()?;
                            TokenData::Comment(comment)
                        }
                        // /* */注释
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
                        // /= 计算
                        '=' => TokenData::Punctuator(Punctuator::AssignDiv),
                        _ => TokenData::Punctuator(Punctuator::Div),
                    };
                    self.push_token(token)
                }
                '*' => op!(self, Punctuator::AssignMul, Punctuator::Mul),
                '+' => op!(self, Punctuator::AssignAdd, Punctuator::Add, {
                    '+' => Punctuator::Inc
                }),
                '-' => op!(self, Punctuator::AssignSub, Punctuator::Sub, {
                    '-' => {
                        self.next()?;
                        Punctuator::Dec
                    }
                }),
                '%' => op!(self, Punctuator::AssignMod, Punctuator::Mod),
                '|' => op!(self, Punctuator::AssignOr, Punctuator::Or, {
                    '|' => Punctuator::BoolOr
                }),
                '&' => op!(self, Punctuator::AssignAnd, Punctuator::And, {
                    '&' => Punctuator::BoolAnd
                }),
                '^' => op!(self, Punctuator::AssignXor, Punctuator::Xor),
                '=' => op!(self, if self.next_is('=')? {
                    Punctuator::StrictEq
                } else {
                    Punctuator::Eq
                }, Punctuator::Assign, {
                    '>' => {
                        self.next()?;
                        Punctuator::Arrow
                    }
                }),
                '<' => op!(self, Punctuator::LessThanOrEq, Punctuator::LessThan, {
                    '<' => vop!(self, Punctuator::AssignLeftSh, Punctuator::LeftSh)
                }),
                '>' => op!(self, Punctuator::GreaterThanOrEq, Punctuator::GreaterThan, {
                    '>' => vop!(self, Punctuator::AssignRightSh, Punctuator::RightSh, {
                        '>' => vop!(self, Punctuator::AssignURightSh, Punctuator::URightSh)
                    })
                }),
                '!' => op!(
                    self,
                    vop!(self, Punctuator::StrictNotEq, Punctuator::NotEq),
                    Punctuator::Not
                ),
                '~' => self.push_punc(Punctuator::Neg),
                '\n' | '\u{2028}' | '\u{2029}' => {
                    self.line_number += 1;
                    self.column_number = 0;
                }
                '\r' => {
                    self.column_number = 0;
                }
                ' ' => (),
                ch => panic!(
                    "{}:{}: Unexpected '{}'",
                    self.line_number, self.column_number, ch
                ),
            }
        }
    }

    fn next(&mut self) -> Result<char, LexerError>{
        match self.buffer.next() {
            Some(char) => Ok(char),
            None => Err(LexerError::new("finished")),
        }
    }

    fn preview_next(&mut self) -> Result<char, LexerError> {
        match self.buffer.peek() {
            Some(v) => Ok(*v),
            None => Err(LexerError::new("finished")),
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



#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::ast::keyword::Keyword;

    #[test]
    fn check_variable_definition_tokens() {
        let s = &String::from("let a = 'hello';");
        let mut lexer = Lexer::new(s);
        lexer.lex().expect("finished");
        assert_eq!(lexer.tokens[0].data, TokenData::Keyword(Keyword::Let));
        assert_eq!(lexer.tokens[1].data, TokenData::Identifier("a".to_string()));
        assert_eq!(
            lexer.tokens[2].data,
            TokenData::Punctuator(Punctuator::Assign)
        );
        assert_eq!(
            lexer.tokens[3].data,
            TokenData::StringLiteral("hello".to_string())
        );
    }

    #[test]
    fn check_positions() {
        let s = &String::from("console.log(\"hello world\");");
        // -------------------123456789
        let mut lexer = Lexer::new(s);
        lexer.lex().expect("finished");
        // The first column is 1 (not zero indexed)
        assert_eq!(lexer.tokens[0].pos.column_number, 1);
        assert_eq!(lexer.tokens[0].pos.line_number, 1);
        // Dot Token starts on line 7
        assert_eq!(lexer.tokens[1].pos.column_number, 8);
        assert_eq!(lexer.tokens[1].pos.line_number, 1);
        // Log Token starts on line 7
        assert_eq!(lexer.tokens[2].pos.column_number, 9);
        assert_eq!(lexer.tokens[2].pos.line_number, 1);
        // Open parenthesis token starts on line 12
        assert_eq!(lexer.tokens[3].pos.column_number, 12);
        assert_eq!(lexer.tokens[3].pos.line_number, 1);
        // String token starts on line 13
        assert_eq!(lexer.tokens[4].pos.column_number, 13);
        assert_eq!(lexer.tokens[4].pos.line_number, 1);
        // Close parenthesis token starts on line 26
        assert_eq!(lexer.tokens[5].pos.column_number, 26);
        assert_eq!(lexer.tokens[5].pos.line_number, 1);
        // Semi Colon token starts on line 27
        assert_eq!(lexer.tokens[6].pos.column_number, 27);
        assert_eq!(lexer.tokens[6].pos.line_number, 1);
    }

    // Increment/Decrement
    #[test]
    fn check_decrement_advances_lexer_2_places() {
        // Here we want an example of decrementing an integer
        let s = &String::from("let a = b--;");
        let mut lexer = Lexer::new(s);
        lexer.lex().expect("finished");
        assert_eq!(lexer.tokens[4].data, TokenData::Punctuator(Punctuator::Dec));
        // Decrementing means adding 2 characters '--', the lexer should consume it as a single token
        // and move the curser forward by 2, meaning the next token should be a semicolon
        assert_eq!(
            lexer.tokens[5].data,
            TokenData::Punctuator(Punctuator::Semicolon)
        );
    }

}