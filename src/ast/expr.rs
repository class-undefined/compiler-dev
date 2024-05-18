use std::fmt::Debug;

use crate::ast::op::Op;

pub enum Expr {
    Number(i32),
    UnaryOp(Op, Box<Expr>),
    BinaryOp(Box<Expr>, Op, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::UnaryOp(op, expr) => write!(f, "({:?}{:?})", op, expr),
            Expr::BinaryOp(lhs, op, rhs) => write!(f, "({:?}{:?}{:?})", lhs, op, rhs),
        }
    }
}
