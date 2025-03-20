use crate::{random::RNG, values::ValTree};

mod num;
pub use num::*;
use rand::Rng;

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

    pub fn random_expr<const M: i32>() -> ExprTree {
        match RNG.with(|rng| rng.borrow_mut().random_range(1..=1)) {
            1 => ExprTree::Num(NumExpr::random::<M>(0)),
            _ => unimplemented!(),
        }
    }

    pub fn mutated(self) -> Self {
        match self {
            ExprTree::Num(num_expr) => ExprTree::Num(num_expr.mutated()),
        }
    }
}
