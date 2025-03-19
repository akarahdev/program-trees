#[derive(Clone, Copy, PartialEq)]
pub struct Number {
    inner: f64,
}

impl Number {
    pub fn new(number: f64) -> Number {
        Number { inner: number }
    }

    pub fn add(&self, other: Number) -> Number {
        Number {
            inner: self.inner + other.inner,
        }
    }

    pub fn sub(&self, other: Number) -> Number {
        Number {
            inner: self.inner - other.inner,
        }
    }

    pub fn mul(&self, other: Number) -> Number {
        Number {
            inner: self.inner * other.inner,
        }
    }

    pub fn div(&self, other: Number) -> Number {
        Number {
            inner: self.inner / other.inner,
        }
    }
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
