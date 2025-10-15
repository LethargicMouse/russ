// provides functions to fatally fail with an error
use std::{fmt::Display, process::exit};

use crate::either::Either;

pub trait OrDie: Either + Sized
where
    Self::Left: Display,
{
    fn or_die(self) -> Self::Right {
        self.either(die, |r| r)
    }
}

impl<T> OrDie for T
where
    T: Either,
    T::Left: Display,
{
}

pub fn die<T>(e: impl Display) -> T {
    eprintln!("{e}");
    exit(1)
}
