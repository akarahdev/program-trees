mod num;
pub use num::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValTree {
    Number(Number),
}
