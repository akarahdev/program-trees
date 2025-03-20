use std::cell::RefCell;

use rand::{Rng, SeedableRng, rngs::SmallRng};

thread_local! {
    pub static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_os_rng());
}

pub trait F64Random {
    fn random() -> f64 {
        RNG.with_borrow_mut(|rng| rng.random_range(-1.0..1.0))
    }
}

impl F64Random for f64 {}
