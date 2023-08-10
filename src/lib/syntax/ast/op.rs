use std::fmt::{Display, Formatter, Result};


#[derive(Clone, PartialEq)]
pub enum NumOp {
    // +
    Add,
    // -
    Sub,
    // /
    Div,
    // *
    Mul,
    // %
    Mod
}

impl Display for NumOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        match *self {
            NumOp::Add => write!(f, "+"),
            NumOp::Sub => write!(f, "-"),
            NumOp::Div => write!(f, "/"),
            NumOp::Mul => write!(f, "*"),
            NumOp::Mod => write!(f, "%"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum UnaryOp {
    /// a++
    IncrementPost,
    /// ++a
    IncrementPre,
    /// a--
    DecrementPost,
    /// --a
    DecrementPre,
    /// -a
    Minus,
    /// +a
    Plus,
    /// !a
    Not,
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match *self {
                UnaryOp::IncrementPost | UnaryOp::IncrementPre => "++",
                UnaryOp::DecrementPost | UnaryOp::DecrementPre => "--",
                UnaryOp::Plus => "+",
                UnaryOp::Minus => "-",
                UnaryOp::Not => "!",
            }
        )
    }
}

#[derive(Clone, PartialEq)]
pub enum BitOp {
    /// `a & b`
    And,
    /// `a | b`
    Or,
    /// `a ^ b`
    Xor,
    /// `a << b`
    Shl,
    /// `a >> b`
    Shr,
}

impl Display for BitOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match *self {
                BitOp::And => "&",
                BitOp::Or => "|",
                BitOp::Xor => "^",
                BitOp::Shl => "<<",
                BitOp::Shr => ">>",
            }
        )
    }
}

#[derive(Clone, PartialEq)]
pub enum CompOp {
    /// `a == b`
    Equal,
    /// `a != b`
    NotEqual,
    /// `a === b`
    StrictEqual,
    /// `a !== b`
    StrictNotEqual,
    /// `a > b`
    GreaterThan,
    /// `a >= b`
    GreaterThanOrEqual,
    /// `a < b`
    LessThan,
    /// `a <= b`
    LessThanOrEqual,
}

impl Display for CompOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match *self {
                CompOp::Equal => "==",
                CompOp::NotEqual => "!=",
                CompOp::StrictEqual => "===",
                CompOp::StrictNotEqual => "!==",
                CompOp::GreaterThan => ">",
                CompOp::GreaterThanOrEqual => ">=",
                CompOp::LessThan => "<",
                CompOp::LessThanOrEqual => "<=",
            }
        )
    }
}

#[derive(Clone, PartialEq)]
pub enum LogOp {
    /// `a && b`
    And,
    /// `a || b`
    Or,
}

impl Display for LogOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match *self {
                LogOp::And => "&&",
                LogOp::Or => "||",
            }
        )
    }
}

/// 二元操作符
#[derive(Clone, PartialEq)]
pub enum BinOp {
    Num(NumOp),
    Bit(BitOp),
    Comp(CompOp),
    Log(LogOp)
}

/// 操作符
pub trait Operator {
    /// 结合
    fn get_assoc(&self) -> bool;
    /// 优先级
    fn get_precedence(&self) -> u64;
    /// 返回两者
    fn get_precedence_and_assoc(&self) -> (u64, bool) {
        (self.get_precedence(), self.get_assoc())
    }
}

impl Operator for BinOp {
    fn get_assoc(&self) -> bool {
        true
    }

    fn get_precedence(&self) -> u64 {
        match *self {
            BinOp::Num(NumOp::Mul) | BinOp::Num(NumOp::Div) | BinOp::Num(NumOp::Mod) => 5,
            BinOp::Num(NumOp::Add) | BinOp::Num(NumOp::Sub) => 6,
            BinOp::Bit(BitOp::Shl) | BinOp::Bit(BitOp::Shr) => 7,
            BinOp::Comp(CompOp::LessThan)
            | BinOp::Comp(CompOp::LessThanOrEqual)
            | BinOp::Comp(CompOp::GreaterThan)
            | BinOp::Comp(CompOp::GreaterThanOrEqual) => 8,
            BinOp::Comp(CompOp::Equal)
            | BinOp::Comp(CompOp::NotEqual)
            | BinOp::Comp(CompOp::StrictEqual)
            | BinOp::Comp(CompOp::StrictNotEqual) => 9,
            BinOp::Bit(BitOp::And) => 10,
            BinOp::Bit(BitOp::Xor) => 11,
            BinOp::Bit(BitOp::Or) => 12,
            BinOp::Log(LogOp::And) => 13,
            BinOp::Log(LogOp::Or) => 14,
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match *self {
            BinOp::Num(ref op) => op.to_string(),
            BinOp::Bit(ref op) => op.to_string(),
            BinOp::Comp(ref op) => op.to_string(),
            BinOp::Log(ref op) => op.to_string(),
        })
    }
}