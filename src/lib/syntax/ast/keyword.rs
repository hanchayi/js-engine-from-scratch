use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};
use std::error;
use super::keyword::Keyword::*;

/**
 * https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar#keywords
 */
 #[derive(Clone, PartialEq, Debug)]
 /// 关键词枚举
pub enum Keyword {
    /// 关键词break
    KBreak,
    /// 关键词case
    KCase,
    /// 关键词catch
    KCatch,
    /// 关键词class, which is reserved for future use
    KClass,
    /// 关键词continue
    KContinue,
    /// 关键词debugger
    KDebugger,
    /// 关键词default
    KDefault,
    /// 关键词delete
    KDelete,
    /// 关键词do
    KDo,
    /// 关键词else
    KElse,
    /// 关键词enum
    KEnum,
    /// 关键词extends
    KExtends,
    /// 关键词finally
    KFinally,
    /// 关键词for
    KFor,
    /// 关键词function
    KFunction,
    /// 关键词if
    KIf,
    /// 关键词in
    KIn,
    /// 关键词instanceof
    KInstanceOf,
    /// 关键词import
    KImport,
    /// 关键词new
    KNew,
    /// 关键词return
    KReturn,
    /// 关键词super
    KSuper,
    /// 关键词switch
    KSwitch,
    /// 关键词this
    KThis,
    /// 关键词throw
    KThrow,
    /// 关键词try
    KTry,
    /// 关键词typeof
    KTypeOf,
    /// 关键词var
    KVar,
    /// 关键词void
    KVoid,
    /// 关键词while
    KWhile,
    /// 关键词with
    KWith,
}

#[derive(Debug, Clone)]
pub struct TokenError;
impl Display for TokenError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "invalid token")
    }
}

// 为了其它Error来wrap它
impl error::Error for TokenError {
    fn description(&self) -> &str {
        "invalid token"
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl FromStr for Keyword {
    type Err = TokenError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "break" => Ok(KBreak),
            "case" => Ok(KCase),
            "catch" => Ok(KCatch),
            "class" => Ok(KClass),
            "continue" => Ok(KContinue),
            "debugger" => Ok(KDebugger),
            "default" => Ok(KDefault),
            "delete" => Ok(KDelete),
            "do" => Ok(KDo),
            "else" => Ok(KElse),
            "enum" => Ok(KEnum),
            "extends" => Ok(KExtends),
            "finally" => Ok(KFinally),
            "for" => Ok(KFor),
            "function" => Ok(KFunction),
            "if" => Ok(KIf),
            "in" => Ok(KIn),
            "instanceof" => Ok(KInstanceOf),
            "import" => Ok(KImport),
            "new" => Ok(KNew),
            "return" => Ok(KReturn),
            "super" => Ok(KSuper),
            "switch" => Ok(KSwitch),
            "this" => Ok(KThis),
            "throw" => Ok(KThrow),
            "try" => Ok(KTry),
            "typeof" => Ok(KTypeOf),
            "var" => Ok(KVar),
            "void" => Ok(KVoid),
            "while" => Ok(KWhile),
            "with" => Ok(KWith),
            _ => Err(TokenError),
        }
    }

}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "{}",
            match *self {
                KBreak => "break",
                KCase => "case",
                KCatch => "catch",
                KClass => "class",
                KContinue => "continue",
                KDebugger => "debugger",
                KDefault => "default",
                KDelete => "delete",
                KDo => "do",
                KElse => "else",
                KEnum => "enum",
                KExtends => "extends",
                KFinally => "finally",
                KFor => "for",
                KFunction => "function",
                KIf => "if",
                KIn => "in",
                KInstanceOf => "instanceof",
                KImport => "import",
                KNew => "new",
                KReturn => "return",
                KSuper => "super",
                KSwitch => "switch",
                KThis => "this",
                KThrow => "throw",
                KTry => "try",
                KTypeOf => "typeof",
                KVar => "var",
                KVoid => "void",
                KWhile => "while",
                KWith => "with",
            }
        )
    }
}