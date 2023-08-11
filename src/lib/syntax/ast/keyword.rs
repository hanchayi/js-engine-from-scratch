use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};
use std::error;
use crate::syntax::ast::keyword::Keyword::*;

/**
 * https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar#keywords
 */
 #[derive(Clone, PartialEq, Debug)]
 /// 关键词枚举
pub enum Keyword {
    /// 关键词break
    Break,
    /// 关键词case
    Case,
    /// 关键词catch
    Catch,
    /// 关键词class, which is reserved for future use
    Class,
    /// 关键词continue
    Continue,
    /// 关键词debugger
    Debugger,
    /// 关键词default
    Default,
    /// 关键词delete
    Delete,
    /// 关键词do
    Do,
    /// 关键词else
    Else,
    /// 关键词enum
    Enum,
    /// 关键词extends
    Extends,
    /// 关键词finally
    Finally,
    /// 关键词for
    For,
    /// 关键词function
    Function,
    /// 关键词if
    If,
    /// 关键词in
    In,
    /// 关键词instanceof
    InstanceOf,
    /// 关键词import
    Import,
    /// 关键词new
    New,
    /// 关键词return
    Return,
    /// 关键词super
    Super,
    /// 关键词switch
    Switch,
    /// 关键词this
    This,
    /// 关键词throw
    Throw,
    /// 关键词try
    Try,
    /// 关键词typeof
    TypeOf,
    /// 关键词var
    Var,
    /// 关键词let
    Let,
    /// 关键词void
    Void,
    /// 关键词while
    While,
    /// 关键词with
    With,
}

#[derive(Debug, Clone)]
pub struct KeywordError;
impl Display for KeywordError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "invalid token")
    }
}

// 为了其它Error来wrap它
impl error::Error for KeywordError {
    fn description(&self) -> &str {
        "invalid token"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl FromStr for Keyword {
    type Err = KeywordError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "break" =>      Ok(Break),
            "case" =>       Ok(Case),
            "catch" =>      Ok(Catch),
            "class" =>      Ok(Class),
            "continue" =>   Ok(Continue),
            "debugger" =>   Ok(Debugger),
            "default" =>    Ok(Default),
            "delete" =>     Ok(Delete),
            "do" =>         Ok(Do),
            "else" =>       Ok(Else),
            "enum" =>       Ok(Enum),
            "extends" =>    Ok(Extends),
            "finally" =>    Ok(Finally),
            "for" =>        Ok(For),
            "function" =>   Ok(Function),
            "if" =>         Ok(If),
            "in" =>         Ok(In),
            "instanceof" => Ok(InstanceOf),
            "import" =>     Ok(Import),
            "new" =>        Ok(New),
            "return" =>     Ok(Return),
            "super" =>      Ok(Super),
            "switch" =>     Ok(Switch),
            "this" =>       Ok(This),
            "throw" =>      Ok(Throw),
            "try" =>        Ok(Try),
            "typeof" =>     Ok(TypeOf),
            "var" =>        Ok(Var),
            "let" =>        Ok(Let),
            "void" =>       Ok(Void),
            "while" =>      Ok(While),
            "with" =>       Ok(With),
            _ =>            Err(KeywordError),
        }
    }

}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match *self {
                Break => "break",
                Case => "case",
                Catch => "catch",
                Class => "class",
                Continue => "continue",
                Debugger => "debugger",
                Default => "default",
                Delete => "delete",
                Do => "do",
                Else => "else",
                Enum => "enum",
                Extends => "extends",
                Finally => "finally",
                For => "for",
                Function => "function",
                If => "if",
                In => "in",
                InstanceOf => "instanceof",
                Import => "import",
                New => "new",
                Return => "return",
                Super => "super",
                Switch => "switch",
                This => "this",
                Throw => "throw",
                Try => "try",
                TypeOf => "typeof",
                Var => "var",
                Void => "void",
                While => "while",
                With => "with",
                Let => "let",
            }
        )
    }
}