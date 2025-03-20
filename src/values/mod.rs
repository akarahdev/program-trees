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
}
