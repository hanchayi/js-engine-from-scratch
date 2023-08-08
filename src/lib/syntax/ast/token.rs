use std::fmt::{Display, Result, Formatter};

use super::keyword::Keyword;
use super::punc::Punctuator;
use super::pos::Position;

#[derive(Clone, PartialEq)]
pub struct Token {
    pub data: TokenData,
    pub pos: Position,
}

// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar
#[derive(Clone, PartialEq)]
pub enum TokenData {
    /// A boolean literal, which is either `true` or `false`
    TBooleanLiteral(bool),
    /// The end of the file
    TEOF,
    /// An identifier
    TIdentifier(String),
    /// A keyword
    TKeyword(Keyword),
    /// A `null` literal
    TNullLiteral,
    /// A numeric literal
    TNumericLiteral(f64),
    /// A piece of punctuation
    TPunctuator(Punctuator),
    /// A string literal
    TStringLiteral(String),
    /// A regular expression
    TRegularExpression(String),
    /// A comment
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