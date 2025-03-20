mod num;
pub use num::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValTree {
    Number(Number),
}

impl ValTree {
    pub fn as_f64(&self) -> f64 {
        match self {
            ValTree::Number(number) => number.as_f64(),
        }
    }

    pub fn add(&self, other: ValTree) -> ValTree {
        match self {
            ValTree::Number(number) => ValTree::Number(number.add(Number::new(other.as_f64()))),
        }
    }

    pub fn sub(&self, other: ValTree) -> ValTree {
        match self {
            ValTree::Number(number) => ValTree::Number(number.sub(Number::new(other.as_f64()))),
        }
    }

    pub fn mul(&self, other: ValTree) -> ValTree {
        match self {
            ValTree::Number(number) => ValTree::Number(number.mul(Number::new(other.as_f64()))),
        }
    }

    pub fn div(&self, other: ValTree) -> ValTree {
        match self {
            ValTree::Number(number) => ValTree::Number(number.div(Number::new(other.as_f64()))),
        }
    }

    pub fn pow(&self, other: ValTree) -> ValTree {
        match self {
            ValTree::Number(number) => ValTree::Number(number.pow(Number::new(other.as_f64()))),
        }
    }
}
