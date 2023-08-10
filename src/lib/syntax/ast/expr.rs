use std::{fmt::{Display, Formatter, Result}, collections::BTreeMap};

use super::{pos::Position, op::{BinOp, UnaryOp}, constant::Constant};

#[derive(PartialEq, Clone)]
pub struct Expr {
 pub def: ExprDef,
 pub start: Position,
 pub end: Position,
}

impl Expr {
    pub fn new(def: ExprDef, start: Position, end: Position) -> Expr {
        Expr {
            def: def,
            start,
            end,
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.def)
    }
}

#[derive(PartialEq, Clone)]
/// Javascript表达式定义
pub enum ExprDef {
    // 二元计算
    BinOpExpr(BinOp, Box<Expr>, Box<Expr>),
    // 一元操作
    UnaryOpExpr(UnaryOp, Box<Expr>),
    // 常量值
    ConstExpr(Constant),
    // new aa(...)
    ConstructExpr(Box<Expr>, Vec<Expr>),
    // {....}
    BlockExpr(Vec<Expr>),
    LocalExpr(String),
    GetConstFieldExpr(Box<Expr>, String),
    GetFieldExpr(Box<Expr>, Box<Expr>),
    // a.fun(....)
    CallExpr(Box<Expr>, Vec<Expr>),
    WhileLoopExpr(Box<Expr>, Box<Expr>),
    IfExpr(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    SwitchExpr(Box<Expr>, Vec<(Expr, Vec<Expr>)>, Option<Box<Expr>>),
    // {a: {}}
    ObjectDeclExpr(Box<BTreeMap<String, Expr>>),
    ArrayDeclExpr(Vec<Expr>),
    FunctionDeclExpr(Option<String>, Vec<String>, Box<Expr>),
    ArrowFunctionDeclExpr(Vec<String>, Box<Expr>),
    ReturnExpr(Option<Box<Expr>>),
    ThrowExpr(Box<Expr>),
    AssignExpr(Box<Expr>, Box<Expr>),
    VarDeclExpr(Vec<(String, Option<Expr>)>),
    TypeOfExpr(Box<Expr>),
}

impl Display for ExprDef {
    fn fmt(&self, f: &mut Formatter) -> Result {
        return match *self {
            ExprDef::ConstExpr(ref c) => write!(f, "{}", c),
            ExprDef::BlockExpr(ref block) => {
                write!(f, "{}", "{")?;
                for expr in block.iter() {
                    write!(f, "{};", expr)?;
                }
                write!(f, "{}", "}")
            }
            ExprDef::LocalExpr(ref s) => write!(f, "{}", s),
            ExprDef::GetConstFieldExpr(ref ex, ref field) => write!(f, "{}.{}", ex, field),
            ExprDef::GetFieldExpr(ref ex, ref field) => write!(f, "{}[{}]", ex, field),
            ExprDef::CallExpr(ref ex, ref args) => {
                write!(f, "{}(", ex)?;
                let arg_strs: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
                write!(f, "{})", arg_strs.connect(","))
            }
            ExprDef::ConstructExpr(ref func, ref args) => write!(f, "new {}({})", func, args),
            ExprDef::WhileLoopExpr(ref cond, ref expr) => write!(f, "while({}) {}", cond, expr),
            ExprDef::IfExpr(ref cond, ref expr, None) => write!(f, "if({}) {}", cond, expr),
            ExprDef::IfExpr(ref cond, ref expr, Some(ref else_e)) => {
                write!(f, "if({}) {} else {}", cond, expr, else_e)
            }
            ExprDef::SwitchExpr(ref val, ref vals, None) => write!(f, "switch({}){}", val, vals),
            ExprDef::SwitchExpr(ref val, ref vals, Some(ref def)) => {
                write!(f, "switch({}){}default:{}", val, vals, def)
            }
            ExprDef::ObjectDeclExpr(ref map) => write!(f, "{}", map),
            ExprDef::ArrayDeclExpr(ref arr) => write!(f, "{}", arr),
            ExprDef::FunctionDeclExpr(ref name, ref args, ref expr) => {
                write!(f, "function {}({}){}", name, args.connect(", "), expr)
            }
            ExprDef::ArrowFunctionDeclExpr(ref args, ref expr) => {
                write!(f, "({}) => {}", args.connect(", "), expr)
            }
            ExprDef::BinOpExpr(ref op, ref a, ref b) => write!(f, "{} {} {}", a, op, b),
            ExprDef::UnaryOpExpr(ref op, ref a) => write!(f, "{}{}", op, a),
            ExprDef::ReturnExpr(Some(ref ex)) => write!(f, "return {}", ex),
            ExprDef::ReturnExpr(None) => write!(f, "{}", "return"),
            ExprDef::ThrowExpr(ref ex) => write!(f, "throw {}", ex),
            ExprDef::AssignExpr(ref ref_e, ref val) => write!(f, "{} = {}", ref_e, val),
            ExprDef::VarDeclExpr(ref vars) => write!(f, "var {}", vars),
            ExprDef::TypeOfExpr(ref e) => write!(f, "typeof {}", e),
        };
    }
}