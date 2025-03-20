use ordered_float::Pow;

#[derive(Clone, PartialEq)]
pub enum ValTree {
    Number(f64),
}

impl std::fmt::Debug for ValTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(value) => write!(f, "{}", value),
        }
    }
}

impl ValTree {
    pub fn as_number(&self) -> f64 {
        match self {
            ValTree::Number(number) => *number,
        }
    }

    pub fn add(&self, other: ValTree) -> ValTree {
        match self {
            ValTree::Number(number) => ValTree::Number(number + other.as_number()),
        }
    }

    pub fn sub(&self, other: ValTree) -> ValTree {
        match self {
            ValTree::Number(number) => ValTree::Number(number - other.as_number()),
        }
    }

    pub fn mul(&self, other: ValTree) -> ValTree {
        match self {
            ValTree::Number(number) => ValTree::Number(number * other.as_number()),
        }
    }

    pub fn div(&self, other: ValTree) -> ValTree {
        match self {
            ValTree::Number(number) => ValTree::Number(number / other.as_number()),
        }
    }

    pub fn pow(&self, other: ValTree) -> ValTree {
        match self {
            ValTree::Number(number) => ValTree::Number(number.pow(other.as_number())),
        }
    }
}
