use std::cell::RefCell;

use rand::{SeedableRng, rngs::SmallRng};

thread_local! {
    pub static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::from_os_rng());
}
