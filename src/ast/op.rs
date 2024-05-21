use std::fmt::Debug;

pub enum Op {
    Add,
    Sub,
    Neg,
    Not,
    Mul,
    Div,
    Mod,
    Gt,  // 大于
    Lt,  // 小于
    Ge,  // 大于等于
    Le,  // 小于等于
    Eq,  // 等于
    Ne,  // 不等于
    And, // 与
    Or,  // 或
    Xor, // 异或
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
            Op::Gt => write!(f, ">"),
            Op::Lt => write!(f, "<"),
            Op::Ge => write!(f, ">="),
            Op::Le => write!(f, "<="),
            Op::Eq => write!(f, "=="),
            Op::Ne => write!(f, "!="),
            Op::And => write!(f, "&&"),
            Op::Or => write!(f, "||"),
            Op::Xor => write!(f, "^"),
        }
    }
}
