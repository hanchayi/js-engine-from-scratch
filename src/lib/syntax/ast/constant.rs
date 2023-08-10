use std::fmt::Display;

// PartialEq部分相等
// Debug -> :?  Display -> {}
#[derive(PartialEq, Debug, Clone)]
/// Javascript 常量
pub enum Const {
    String(String),
    Num(f64),
    Int(i32),
    RegExp(String, bool, bool),
    Bool(bool),
    Null,
    Undefined,
}

impl Display for Const {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match *self {
            // ref是借用没有移动
            Const::String(ref s) => write!(f, "\"{}\"", s),
            Const::Num(n) => write!(f, "{}", n),
            Const::Int(i) => write!(f, "{}", i),
            Const::RegExp(ref reg, _, _) => write!(f, "{}", reg),
            Const::Bool(b) => write!(f, "{}", b),
            Const::Null => write!(f, "null"),
            Const::Undefined => write!(f, "undefined"),
        }
    }
}