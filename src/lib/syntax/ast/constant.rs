use std::fmt::Display;

// PartialEq部分相等
// Debug -> :?  Display -> {}
#[derive(PartialEq, Debug, Clone)]
/// Javascript 常量
pub enum Constant {
    String(String),
    Num(f64),
    Int(i32),
    RegExp(String, bool, bool),
    Boalean(bool),
    Null,
    Undefined,
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match *self {
            // ref是借用没有移动
            Constant::String(ref s) => write!(f, "\"{}\"", s),
            Constant::Num(n) => write!(f, "{}", n),
            Constant::Int(i) => write!(f, "{}", i),
            Constant::RegExp(ref reg, _, _) => write!(f, "{}", reg),
            Constant::Boalean(b) => write!(f, "{}", b),
            Constant::Null => write!(f, "null"),
            Constant::Undefined => write!(f, "undefined"),
        }
    }
}