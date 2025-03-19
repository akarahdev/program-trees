#[derive(Debug, Clone, Copy, PartialEq)]
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
