use rand::Rng;

use crate::{
    IntoBox,
    random::{F64Random, RNG},
    values::ValTree,
};

#[derive(Clone, PartialEq)]
pub enum ExprTree {
    Index(usize),
    Constant(ValTree),
    Add(Box<ExprTree>, Box<ExprTree>),
    Sub(Box<ExprTree>, Box<ExprTree>),
    Mul(Box<ExprTree>, Box<ExprTree>),
    Div(Box<ExprTree>, Box<ExprTree>),
    Pow(Box<ExprTree>, Box<ExprTree>),
}

impl ExprTree {
    pub fn eval(&self, input: &[ValTree]) -> ValTree {
        match self {
            ExprTree::Index(idx) => {
                ValTree::Number(input.get(*idx).map(|x| x.as_number()).unwrap_or(0.0))
            }
            ExprTree::Constant(value) => value.clone(),
            ExprTree::Add(left, right) => left.eval(input).add(right.eval(input)),
            ExprTree::Sub(left, right) => left.eval(input).sub(right.eval(input)),
            ExprTree::Mul(left, right) => left.eval(input).mul(right.eval(input)),
            ExprTree::Div(left, right) => left.eval(input).div(right.eval(input)),
            ExprTree::Pow(left, right) => left.eval(input).pow(right.eval(input)),
        }
    }

    pub fn priority(&self) -> i32 {
        match self {
            ExprTree::Index(_) => 8,
            ExprTree::Constant(_) => 4,
            ExprTree::Add(_, _) => 1,
            ExprTree::Sub(_, _) => 1,
            ExprTree::Mul(_, _) => 2,
            ExprTree::Div(_, _) => 2,
            ExprTree::Pow(_, _) => 3,
        }
    }

    pub fn random<const M: i32>(depth: i32) -> ExprTree {
        if depth > M {
            return ExprTree::Constant(ValTree::Number(f64::random()));
        }
        match RNG.with(|rng| rng.borrow_mut().random_range(1..=9)) {
            1..=4 => ExprTree::Constant(ValTree::Number(f64::random())),
            5 => ExprTree::Add(
                Self::random::<M>(depth + 1).boxed(),
                Self::random::<M>(depth + 1).boxed(),
            ),
            6 => ExprTree::Sub(
                Self::random::<M>(depth + 1).boxed(),
                Self::random::<M>(depth + 1).boxed(),
            ),
            7 => ExprTree::Mul(
                Self::random::<M>(depth + 1).boxed(),
                Self::random::<M>(depth + 1).boxed(),
            ),
            8 => ExprTree::Div(
                Self::random::<M>(depth + 1).boxed(),
                Self::random::<M>(depth + 1).boxed(),
            ),
            9 => ExprTree::Index(RNG.with(|rng| rng.borrow_mut().random_range(0..=2))),
            _ => unimplemented!(),
        }
    }

    pub fn mutated(self) -> ExprTree {
        match self {
            ExprTree::Index(idx) => match RNG.with(|rng| rng.borrow_mut().random_range(1..=2)) {
                1 => ExprTree::Index(idx),
                2 => ExprTree::Index(RNG.with(|rng| rng.borrow_mut().random_range(0..=2))),
                _ => unreachable!(),
            },
            ExprTree::Constant(number) => {
                match RNG.with(|rng| rng.borrow_mut().random_range(1..=4)) {
                    1 => ExprTree::Constant(number),
                    2 => ExprTree::Constant(
                        number
                            .add(ValTree::Number(f64::random()))
                            .mul(ValTree::Number(f64::random())),
                    ),
                    3 => ExprTree::Add(
                        ExprTree::Constant(number.clone()).boxed(),
                        ExprTree::Constant(number).boxed(),
                    ),
                    4 => ExprTree::Mul(
                        ExprTree::Constant(number.clone()).boxed(),
                        ExprTree::Constant(number).boxed(),
                    ),
                    _ => unreachable!(),
                }
            }
            ExprTree::Add(num_expr, num_expr1) => {
                match RNG.with(|rng| rng.borrow_mut().random_range(1..=6)) {
                    1 => ExprTree::Add(num_expr, num_expr1),
                    2 => ExprTree::Add(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    3 => ExprTree::Sub(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    4 => ExprTree::Mul(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    5 => ExprTree::Div(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    6 => ExprTree::Constant(ValTree::Number(f64::random())),
                    _ => unreachable!(),
                }
            }
            ExprTree::Sub(num_expr, num_expr1) => {
                match RNG.with(|rng| rng.borrow_mut().random_range(1..=7)) {
                    1 => ExprTree::Div(num_expr, num_expr1),
                    2 => ExprTree::Add(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    3 => ExprTree::Sub(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    4 => ExprTree::Mul(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    5 => ExprTree::Div(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    6 => ExprTree::Pow(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    7 => ExprTree::Constant(ValTree::Number(f64::random())),
                    _ => unreachable!(),
                }
            }
            ExprTree::Mul(num_expr, num_expr1) => {
                match RNG.with(|rng| rng.borrow_mut().random_range(1..=7)) {
                    1 => ExprTree::Div(num_expr, num_expr1),
                    2 => ExprTree::Add(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    3 => ExprTree::Sub(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    4 => ExprTree::Mul(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    5 => ExprTree::Div(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    6 => ExprTree::Pow(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    7 => ExprTree::Constant(ValTree::Number(f64::random())),
                    _ => unreachable!(),
                }
            }
            ExprTree::Div(num_expr, num_expr1) => {
                match RNG.with(|rng| rng.borrow_mut().random_range(1..=7)) {
                    1 => ExprTree::Div(num_expr, num_expr1),
                    2 => ExprTree::Add(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    3 => ExprTree::Sub(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    4 => ExprTree::Mul(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    5 => ExprTree::Div(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    6 => ExprTree::Pow(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    7 => ExprTree::Constant(ValTree::Number(f64::random())),
                    _ => unreachable!(),
                }
            }
            ExprTree::Pow(num_expr, num_expr1) => {
                match RNG.with(|rng| rng.borrow_mut().random_range(1..=7)) {
                    1 => ExprTree::Div(num_expr, num_expr1),
                    2 => ExprTree::Add(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    3 => ExprTree::Sub(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    4 => ExprTree::Mul(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    5 => ExprTree::Div(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    6 => ExprTree::Pow(num_expr.mutated().boxed(), num_expr1.mutated().boxed()),
                    7 => ExprTree::Constant(ValTree::Number(f64::random())),
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl ExprTree {
    fn pretty_print(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        parent_priority: i32,
    ) -> std::fmt::Result {
        let my_priority = self.priority();
        let needs_parentheses = my_priority < parent_priority;

        if needs_parentheses {
            write!(f, "(")?;
        }

        match self {
            ExprTree::Constant(num) => write!(f, "{num:?}")?,
            ExprTree::Index(num) => match num {
                0 => write!(f, "x")?,
                1 => write!(f, "z")?,
                _ => write!(f, "0.0")?,
            },
            ExprTree::Add(left, right) => {
                self.pretty_print_op(f, left, right, my_priority, " + ", 0)?
            }
            ExprTree::Sub(left, right) => {
                self.pretty_print_op(f, left, right, my_priority, " - ", 1)?
            }
            ExprTree::Mul(left, right) => {
                self.pretty_print_op(f, left, right, my_priority, " * ", 0)?
            }
            ExprTree::Div(left, right) => {
                self.pretty_print_op(f, left, right, my_priority, " / ", 1)?
            }
            ExprTree::Pow(left, right) => {
                self.pretty_print_op(f, left, right, my_priority, " ^ ", 1)?
            }
        }

        if needs_parentheses {
            write!(f, ")")?;
        }

        Ok(())
    }

    fn pretty_print_op(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        left: &ExprTree,
        right: &ExprTree,
        my_priority: i32,
        op: &str,
        right_adjust: i32,
    ) -> std::fmt::Result {
        left.pretty_print(f, my_priority)?;
        write!(f, "{}", op)?;
        right.pretty_print(f, my_priority + right_adjust)
    }
}

impl std::fmt::Debug for ExprTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_print(f, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::values::ValTree;

    use super::ExprTree;

    #[test]
    pub fn test_add_expr() {
        let expr = ExprTree::Add(
            Box::new(ExprTree::Constant(ValTree::Number(10.0))),
            Box::new(ExprTree::Constant(ValTree::Number(20.0))),
        );
        assert_eq!(expr.eval(&[]), ValTree::Number(30.0));
    }

    #[test]
    pub fn test_sub_expr() {
        let expr = ExprTree::Sub(
            Box::new(ExprTree::Constant(ValTree::Number(10.0))),
            Box::new(ExprTree::Constant(ValTree::Number(40.0))),
        );
        assert_eq!(expr.eval(&[]), ValTree::Number(-30.0));
    }
}
