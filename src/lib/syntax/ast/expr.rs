use std::{fmt::{Display, Formatter, Result}, collections::BTreeMap};

use super::{pos::Position, op::{BinOp, UnaryOp, Operator}, constant::Constant};

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

impl Operator for ExprDef {
    fn get_assoc(&self) -> bool {
        match *self {
            ExprDef::ConstructExpr(_, _)
            | ExprDef::UnaryOpExpr(_, _)
            | ExprDef::TypeOfExpr(_)
            | ExprDef::IfExpr(_, _, _)
            | ExprDef::AssignExpr(_, _) => false,
            _ => true,
        }
    }
    fn get_precedence(&self) -> u64 {
        match self {
            ExprDef::GetFieldExpr(_, _) | ExprDef::GetConstFieldExpr(_, _) => 1,
            ExprDef::CallExpr(_, _) | ExprDef::ConstructExpr(_, _) => 2,
            ExprDef::UnaryOpExpr(UnaryOp::IncrementPost, _)
            | ExprDef::UnaryOpExpr(UnaryOp::IncrementPre, _)
            | ExprDef::UnaryOpExpr(UnaryOp::DecrementPost, _)
            | ExprDef::UnaryOpExpr(UnaryOp::DecrementPre, _) => 3,
            ExprDef::UnaryOpExpr(UnaryOp::Not, _)
            | ExprDef::UnaryOpExpr(UnaryOp::Minus, _)
            | ExprDef::TypeOfExpr(_) => 4,
            ExprDef::BinOpExpr(op, _, _) => op.get_precedence(),
            ExprDef::IfExpr(_, _, _) => 15,
            // 16 should be yield
            ExprDef::AssignExpr(_, _) => 17,
            _ => 19,
        }
    }
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
                write!(f, "{})", arg_strs.join(","))
            }
            ExprDef::ConstructExpr(ref func, ref args) => {
                f.write_fmt(format_args!("new {}", func))?;
                f.write_str("(")?;
                let mut first = true;
                for e in args.iter() {
                    if !first {
                        f.write_str(", ")?;
                    }
                    first = false;
                    Display::fmt(e, f)?;
                }
                f.write_str(")")
            }
            ExprDef::WhileLoopExpr(ref cond, ref expr) => write!(f, "while({}) {}", cond, expr),
            ExprDef::IfExpr(ref cond, ref expr, None) => write!(f, "if({}) {}", cond, expr),
            ExprDef::IfExpr(ref cond, ref expr, Some(ref else_e)) => {
                write!(f, "if({}) {} else {}", cond, expr, else_e)
            }
            ExprDef::SwitchExpr(ref val, ref vals, None) => {
                f.write_fmt(format_args!("switch({})", val))?;
                f.write_str(" {")?;
                for e in vals.iter() {
                    f.write_fmt(format_args!("case {}: \n", e.0))?;
                    join_expr(f, &e.1)?;
                }
                f.write_str("}")
            }
            ExprDef::SwitchExpr(ref val, ref vals, Some(ref def)) => {
                f.write_fmt(format_args!("switch({})", val))?;
                f.write_str(" {")?;
                for e in vals.iter() {
                    f.write_fmt(format_args!("case {}: \n", e.0))?;
                    join_expr(f, &e.1)?;
                }
                f.write_str("default: \n")?;
                Display::fmt(def, f)?;
                f.write_str("}")
            }
            ExprDef::ObjectDeclExpr(ref map) => {
                f.write_str("{")?;
                for (key, value) in map.iter() {
                    f.write_fmt(format_args!("{}: {},", key, value))?;
                }
                f.write_str("}")
            }
            ExprDef::ArrayDeclExpr(ref arr) => {
                f.write_str("[")?;
                join_expr(f, arr)?;
                f.write_str("]")
            }
            ExprDef::FunctionDeclExpr(ref name, ref args, ref expr) => match name {
                Some(val) => write!(f, "function {}({}){}", val, args.join(", "), expr),
                None => write!(f, "function ({}){}", args.join(", "), expr),
            },
            ExprDef::ArrowFunctionDeclExpr(ref args, ref expr) => {
                write!(f, "({}) => {}", args.join(", "), expr)
            }
            ExprDef::BinOpExpr(ref op, ref a, ref b) => write!(f, "{} {} {}", a, op, b),
            ExprDef::UnaryOpExpr(ref op, ref a) => write!(f, "{}{}", op, a),
            ExprDef::ReturnExpr(Some(ref ex)) => write!(f, "return {}", ex),
            ExprDef::ReturnExpr(None) => write!(f, "{}", "return"),
            ExprDef::ThrowExpr(ref ex) => write!(f, "throw {}", ex),
            ExprDef::AssignExpr(ref ref_e, ref val) => write!(f, "{} = {}", ref_e, val),
            ExprDef::VarDeclExpr(ref vars) => {
                f.write_str("var ")?;
                for (key, val) in vars.iter() {
                    match val {
                        Some(x) => f.write_fmt(format_args!("{} = {}", key, x))?,
                        None => f.write_fmt(format_args!("{}", key))?,
                    }
                }
                f.write_str("")
            }
            ExprDef::TypeOfExpr(ref e) => write!(f, "typeof {}", e),
        };
    }
}

fn join_expr(f: &mut Formatter, expr: &Vec<Expr>) -> Result {
    let mut first = true;
    for e in expr.iter() {
        if !first {
            f.write_str(", ")?;
        }
        first = false;
        Display::fmt(e, f)?;
    }
    Ok(())
}