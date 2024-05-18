use std::fmt::Debug;

pub enum Op {
    Add,
    Sub,
    Neg,
    Not,
    Mul,
    Div,
    Mod,
}

impl Debug for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
            Op::Neg => write!(f, "-"),
            Op::Not => write!(f, "!"),
            Op::Mul => write!(f, "*"),
            Op::Div => write!(f, "/"),
            Op::Mod => write!(f, "%"),
        }
    }
}
