use std::fmt::{Display, Result, Formatter};

use super::keyword::Keyword;
use super::punc::Punctuator;
use super::pos::Position;

#[derive(Clone, PartialEq)]
/// js中一个Token
pub struct Token {
    pub data: TokenData,
    pub pos: Position,
}

impl Token {
    /// 通过tokenData和行列号创建Token
    pub fn new(data: TokenData, line_number: u64, column_number: u64) -> Token {
        Token {
            data: data,
            pos: Position::new(line_number, column_number),
        }
    }
}

// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar
#[derive(Clone, PartialEq)]
/// 代表不同类型的Token
pub enum TokenData {
    /// 布尔值
    BooleanLiteral(bool),
    /// 文件结尾
    EOF,
    /// 标志符
    Identifier(String),
    /// 关键词
    Keyword(Keyword),
    /// null
    NullLiteral,
    /// 数字
    NumericLiteral(f64),
    /// 标点符号
    Punctuator(Punctuator),
    /// 字符串
    StringLiteral(String),
    /// 正则
    RegularExpression(String),
    /// 注释
    Comment(String),
}

impl Display for TokenData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.clone() {
            TokenData::BooleanLiteral(val) => write!(f, "{}", val),
            TokenData::EOF => write!(f, "end of file"),
            TokenData::Identifier(iden) => write!(f, "{}", iden),
            TokenData::Keyword(keyword) => write!(f, "{:?}", keyword),
            TokenData::NullLiteral => write!(f, "null"),
            TokenData::NumericLiteral(num) => write!(f, "{}", num),
            TokenData::Punctuator(punctuator) => write!(f, "{:?}", punctuator),
            TokenData::StringLiteral(str) => write!(f, "{}", str),
            TokenData::RegularExpression(regex) => write!(f, "{}", regex),
            TokenData::Comment(comment) => write!(f, "{}", comment),
        }
    }
}