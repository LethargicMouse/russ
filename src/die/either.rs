use std::fmt::Display;

use crate::{die, either::Either};

pub trait OrDie: Either + Sized {
    fn or_die_with<Err: Display>(self, f: impl FnOnce(Self::Left) -> Err) -> Self::Right {
        self.either(|e| die(f(e)), |r| r)
    }
}

impl<T: Either> OrDie for T {}
