#![feature(thread_local)]

pub mod evolution;
pub mod expr;
pub mod random;
pub mod values;

pub trait IntoBox
where
    Self: Sized,
{
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl<T: Sized> IntoBox for T {}
