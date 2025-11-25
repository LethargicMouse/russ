use std::{fmt::Display, fs::File, io};

use crate::die::Mortal;

pub fn create(path: &str) -> File {
    File::create(path).or_die_with(|e| Error(path, e))
}

struct Error<'a>(&'a str, io::Error);

impl Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "! error creating `{}`: {}", self.0, self.1)
    }
}
