use crate::values::ValTree;

mod num;
pub use num::*;
mod random;
pub use random::*;

#[derive(Debug, Clone, PartialEq)]
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
