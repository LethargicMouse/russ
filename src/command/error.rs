use std::{fmt::Display, io};

pub struct Error<'a>(pub &'a str, pub io::Error);

impl Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "! error running command `{}`: {}", self.0, self.1)
    }
}
