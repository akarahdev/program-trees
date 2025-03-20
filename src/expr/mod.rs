use crate::values::ValTree;

mod num;
pub use num::*;

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

    pub fn random_expr() -> ExprTree {
        match rand::random_range(1..=1) {
            1 => ExprTree::Num(NumExpr::random(0)),
            _ => unimplemented!(),
        }
    }

    pub fn mutate(&mut self) {
        match self {
            ExprTree::Num(num_expr) => num_expr.mutate(),
        }
    }
}
