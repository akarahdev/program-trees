use crate::values::ValTree;

mod num;
pub use num::*;
pub enum ExprTree {
    Num(NumExpr),
}

impl ExprTree {
    pub fn eval(&self) -> ValTree {
        match self {
            ExprTree::Num(expr) => ValTree::Number(expr.eval()),
        }
    }
}
