use std::{fmt::Display, io};

pub struct OpenError<'a>(pub &'a str, pub io::Error);

impl Display for OpenError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "! failed to open `{}`: {}", self.0, self.1)
    }
}
