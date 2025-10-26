use std::fmt::Display;

use crate::file;

pub struct Error<Path, Inner>(pub Path, pub Inner);

impl<Path: Display, Inner: Display> Display for Error<Path, Inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", file::Error("open", &self.0, &self.1))
    }
}
