use std::{fmt::Display, process::exit};

use crate::either::Either;

pub fn die<T>(e: impl Display) -> T {
    eprintln!("{e}");
    exit(1)
}

pub trait Mortal: Either + Sized {
    fn or_die_with<E: Display>(self, f: impl FnOnce(Self::Left) -> E) -> Self::Right {
        self.either(|l| die(f(l)), |r| r)
    }
}

impl<T> Mortal for Option<T> {}

impl<R, E> Mortal for Result<R, E> {}
