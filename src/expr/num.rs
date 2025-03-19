use crate::values::Number;

#[derive(Debug, Clone, PartialEq)]
pub enum NumExpr {
    Constant(Number),
    Add(Box<NumExpr>, Box<NumExpr>),
    Sub(Box<NumExpr>, Box<NumExpr>),
    Mul(Box<NumExpr>, Box<NumExpr>),
    Div(Box<NumExpr>, Box<NumExpr>),
}

impl NumExpr {
    pub fn eval(&self) -> Number {
        match self {
            NumExpr::Constant(number) => *number,
            NumExpr::Add(left, right) => left.eval().add(right.eval()),
            NumExpr::Sub(left, right) => left.eval().sub(right.eval()),
            NumExpr::Mul(left, right) => left.eval().mul(right.eval()),
            NumExpr::Div(left, right) => left.eval().div(right.eval()),
        }
    }

    pub fn priority(&self) -> i32 {
        match self {
            NumExpr::Constant(_) => 4,
            NumExpr::Add(_, _) => 1,
            NumExpr::Sub(_, _) => 1,
            NumExpr::Mul(_, _) => 2,
            NumExpr::Div(_, _) => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expr::ExprTree,
        values::{Number, ValTree},
    };

    use super::NumExpr;

    #[test]
    pub fn test_add_expr() {
        let expr = ExprTree::Num(NumExpr::Add(
            Box::new(NumExpr::Constant(Number::new(10.0))),
            Box::new(NumExpr::Constant(Number::new(20.0))),
        ));
        assert_eq!(expr.eval(), ValTree::Number(Number::new(30.0)));
    }

    #[test]
    pub fn test_sub_expr() {
        let expr = ExprTree::Num(NumExpr::Sub(
            Box::new(NumExpr::Constant(Number::new(10.0))),
            Box::new(NumExpr::Constant(Number::new(40.0))),
        ));
        assert_eq!(expr.eval(), ValTree::Number(Number::new(-30.0)));
    }
}
