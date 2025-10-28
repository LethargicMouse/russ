use std::fmt::Display;

use crate::file;

pub struct Error<P, I>(pub P, pub I);

impl<P: Display, I: Display> Display for Error<P, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", file::Error("write to", &self.0, &self.1))
    }
}
