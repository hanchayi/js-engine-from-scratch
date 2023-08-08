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
    TBooleanLiteral(bool),
    /// 文件结尾
    TEOF,
    /// 标志符
    TIdentifier(String),
    /// 关键词
    TKeyword(Keyword),
    /// null
    TNullLiteral,
    /// 数字
    TNumericLiteral(f64),
    /// 标点符号
    TPunctuator(Punctuator),
    /// 字符串
    TStringLiteral(String),
    /// 正则
    TRegularExpression(String),
    /// 注释
    TComment(String),
}

impl Display for TokenData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.clone() {
            TokenData::TBooleanLiteral(val) => write!(f, "{}", val),
            TokenData::TEOF => write!(f, "end of file"),
            TokenData::TIdentifier(iden) => write!(f, "{}", iden),
            TokenData::TKeyword(keyword) => write!(f, "{:?}", keyword),
            TokenData::TNullLiteral => write!(f, "null"),
            TokenData::TNumericLiteral(num) => write!(f, "{}", num),
            TokenData::TPunctuator(punctuator) => write!(f, "{:?}", punctuator),
            TokenData::TStringLiteral(str) => write!(f, "{}", str),
            TokenData::TRegularExpression(regex) => write!(f, "{}", regex),
            TokenData::TComment(comment) => write!(f, "{}", comment),
        }
    }
}