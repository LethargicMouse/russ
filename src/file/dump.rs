use std::{
    fmt::Display,
    io::{self, Write},
};

use crate::{die::Mortal, file::create};

pub fn dump(t: impl Display, path: &str) {
    let mut out = create(path);
    write!(out, "{t}").or_die_with(|e| Error(path, e))
}

struct Error<'a>(&'a str, io::Error);

impl Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "! error writing to `{}`: {}", self.0, self.1)
    }
}
