use rand::Rng;

use crate::{
    IntoBox,
    random::RNG,
    values::{Number, ValTree},
};

#[derive(Debug, Clone, PartialEq)]
pub enum NumExpr {
    Index(usize),
    Constant(Number),
    Add(Box<NumExpr>, Box<NumExpr>),
    Sub(Box<NumExpr>, Box<NumExpr>),
    Mul(Box<NumExpr>, Box<NumExpr>),
    Div(Box<NumExpr>, Box<NumExpr>),
}

impl NumExpr {
    pub fn eval(&self, input: &[ValTree]) -> Number {
        match self {
            NumExpr::Index(idx) => input
                .get(*idx)
                .map(|x| Number::new(x.as_f64()))
                .unwrap_or(Number::new(0.0)),
            NumExpr::Constant(number) => *number,
            NumExpr::Add(left, right) => left.eval(input).add(right.eval(input)),
            NumExpr::Sub(left, right) => left.eval(input).sub(right.eval(input)),
            NumExpr::Mul(left, right) => left.eval(input).mul(right.eval(input)),
            NumExpr::Div(left, right) => left.eval(input).div(right.eval(input)),
        }
    }

    pub fn priority(&self) -> i32 {
        match self {
            NumExpr::Index(_) => 8,
            NumExpr::Constant(_) => 4,
            NumExpr::Add(_, _) => 1,
            NumExpr::Sub(_, _) => 1,
            NumExpr::Mul(_, _) => 2,
            NumExpr::Div(_, _) => 2,
        }
    }

    pub fn random<const M: i32>(depth: i32) -> NumExpr {
        if depth > M {
            return NumExpr::Constant(Number::random());
        }
        match RNG.with(|rng| rng.borrow_mut().random_range(1..=9)) {
            1..=4 => NumExpr::Constant(Number::random()),
            5 => NumExpr::Add(
                Self::random::<M>(depth + 1).boxed(),
                Self::random::<M>(depth + 1).boxed(),
            ),
            6 => NumExpr::Sub(
                Self::random::<M>(depth + 1).boxed(),
                Self::random::<M>(depth + 1).boxed(),
            ),
            7 => NumExpr::Mul(
                Self::random::<M>(depth + 1).boxed(),
                Self::random::<M>(depth + 1).boxed(),
            ),
            8 => NumExpr::Div(
                Self::random::<M>(depth + 1).boxed(),
                Self::random::<M>(depth + 1).boxed(),
            ),
            9 => NumExpr::Index(RNG.with(|rng| rng.borrow_mut().random_range(0..=2))),
            _ => unimplemented!(),
        }
    }

    pub fn mutated(self) -> NumExpr {
        match self {
            NumExpr::Index(idx) => match RNG.with(|rng| rng.borrow_mut().random_range(1..=2)) {
                1 => NumExpr::Index(idx),
                2 => NumExpr::Index(RNG.with(|rng| rng.borrow_mut().random_range(0..=2))),
                _ => unreachable!(),
            },
            NumExpr::Constant(number) => match RNG.with(|rng| rng.borrow_mut().random_range(1..=4))
            {
                1 => self,
                2 => NumExpr::Constant(number.add(Number::random()).mul(Number::random())),
                3 => NumExpr::Add(
                    NumExpr::Constant(number).boxed(),
                    NumExpr::Constant(number).boxed(),
                ),
                4 => NumExpr::Mul(
                    NumExpr::Constant(number).boxed(),
                    NumExpr::Constant(number).boxed(),
                ),
                _ => unreachable!(),
            },
            NumExpr::Add(num_expr, num_expr1) => {
                match RNG.with(|rng| rng.borrow_mut().random_range(1..=4)) {
                    1 => NumExpr::Add(num_expr, num_expr1),
                    2 => NumExpr::Add(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    3 => NumExpr::Sub(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    4 => NumExpr::Constant(Number::random()),
                    _ => unreachable!(),
                }
            }
            NumExpr::Sub(num_expr, num_expr1) => {
                match RNG.with(|rng| rng.borrow_mut().random_range(1..=4)) {
                    1 => NumExpr::Sub(num_expr, num_expr1),
                    2 => NumExpr::Sub(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    3 => NumExpr::Add(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    4 => NumExpr::Constant(Number::random()),
                    _ => unreachable!(),
                }
            }
            NumExpr::Mul(num_expr, num_expr1) => {
                match RNG.with(|rng| rng.borrow_mut().random_range(1..=4)) {
                    1 => NumExpr::Mul(num_expr, num_expr1),
                    2 => NumExpr::Mul(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    3 => NumExpr::Div(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    4 => NumExpr::Constant(Number::random()),
                    _ => unreachable!(),
                }
            }
            NumExpr::Div(num_expr, num_expr1) => {
                match RNG.with(|rng| rng.borrow_mut().random_range(1..=4)) {
                    1 => NumExpr::Div(num_expr, num_expr1),
                    2 => NumExpr::Div(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    3 => NumExpr::Mul(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    4 => NumExpr::Constant(Number::random()),
                    _ => unreachable!(),
                }
            }
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
        assert_eq!(expr.eval(&[]), ValTree::Number(Number::new(30.0)));
    }

    #[test]
    pub fn test_sub_expr() {
        let expr = ExprTree::Num(NumExpr::Sub(
            Box::new(NumExpr::Constant(Number::new(10.0))),
            Box::new(NumExpr::Constant(Number::new(40.0))),
        ));
        assert_eq!(expr.eval(&[]), ValTree::Number(Number::new(-30.0)));
    }
}
