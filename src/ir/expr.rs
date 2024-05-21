use crate::ast::expr::Expr;
use crate::ast::op::Op;
pub struct IRExprGen {
    pub count: usize,
    pub instructions: Vec<String>,
}

impl IRExprGen {
    pub fn new() -> Self {
        IRExprGen {
            count: 0,
            instructions: Vec::new(),
        }
    }

    pub fn gen_temp_id(&mut self) -> String {
        let id = format!("%{}", self.count);
        self.count += 1;
        id
    }

    pub fn gen_ir(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Number(n) => n.to_string(),
            Expr::UnaryOp(op, _expr) => {
                let operand = self.gen_temp_id();
                let value = self.gen_ir(_expr);
                match op {
                    Op::Add => self
                        .instructions
                        .push(format!("{} = add 0, {}", operand, value)),
                    Op::Neg => self
                        .instructions
                        .push(format!("{} = sub 0, {}", operand, value)),
                    Op::Not => self
                        .instructions
                        .push(format!("{} = eq 0, {}", operand, value)),
                    _ => panic!("unreachable"),
                }
                operand
            }
            Expr::BinaryOp(lhs, op, rhs) => {
                let left = self.gen_ir(lhs);
                let right = self.gen_ir(rhs);
                let operand = self.gen_temp_id();
                match op {
                    Op::Add => {
                        self.instructions
                            .push(format!("{} = add {}, {}", operand, left, right));
                    }
                    Op::Sub => {
                        self.instructions
                            .push(format!("{} = sub {}, {}", operand, left, right));
                    }
                    Op::Mul => {
                        self.instructions
                            .push(format!("{} = mul {}, {}", operand, left, right));
                    }
                    Op::Div => {
                        self.instructions
                            .push(format!("{} = div {}, {}", operand, left, right));
                    }
                    Op::Mod => {
                        self.instructions
                            .push(format!("{} = mod {}, {}", operand, left, right));
                    }
                    Op::Eq => {
                        self.instructions
                            .push(format!("{} = eq {}, {}", operand, left, right));
                    }
                    Op::Ne => {
                        self.instructions
                            .push(format!("{} = ne {}, {}", operand, left, right));
                    }
                    Op::Lt => {
                        self.instructions
                            .push(format!("{} = lt {}, {}", operand, left, right));
                    }
                    Op::Le => {
                        self.instructions
                            .push(format!("{} = le {}, {}", operand, left, right));
                    }
                    Op::Gt => {
                        self.instructions
                            .push(format!("{} = gt {}, {}", operand, left, right));
                    }
                    Op::Ge => {
                        self.instructions
                            .push(format!("{} = ge {}, {}", operand, left, right));
                    }
                    _ => panic!("unreachable {:?}", op),
                    // 其他操作
                }
                operand
            }
        }
    }
}

impl Expr {
    pub fn to_ir(&self) -> (usize, String) {
        let mut ir = IRExprGen::new();
        ir.gen_ir(self);
        (ir.count - 1, ir.instructions.join("\n"))
    }
}
