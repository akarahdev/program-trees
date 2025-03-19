use crate::{IntoBox, values::Number};

use super::{ExprTree, NumExpr};

pub fn random_expr() -> ExprTree {
    match rand::random_range(1..=1) {
        1 => ExprTree::Num(random_num_expr()),
        _ => unimplemented!(),
    }
}

pub fn random_num_expr() -> NumExpr {
    match rand::random_range(1..=8) {
        1..=4 => NumExpr::Constant(Number::new(0.0)),
        5 => NumExpr::Add(random_num_expr().boxed(), random_num_expr().boxed()),
        6 => NumExpr::Sub(random_num_expr().boxed(), random_num_expr().boxed()),
        7 => NumExpr::Mul(random_num_expr().boxed(), random_num_expr().boxed()),
        8 => NumExpr::Div(random_num_expr().boxed(), random_num_expr().boxed()),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
pub mod tests {
    use std::time::Duration;

    use super::random_expr;

    #[test]
    pub fn random_exprs() {
        std::thread::sleep(Duration::from_millis(10));
        let expr = random_expr();
        eprintln!("{:?}", expr);
    }
}
