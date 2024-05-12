use std::fmt::Debug;

pub enum Op {
    Add,
    Sub,
    Neg,
    Not,
}

impl Debug for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Neg => write!(f, "-"),
            Op::Not => write!(f, "!"),
        }
    }
}

pub enum Expr {
    Number(i32),
    UnaryOp(Op, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::UnaryOp(op, expr) => write!(f, "({:?}{:?})", op, expr),
        }
    }
}
