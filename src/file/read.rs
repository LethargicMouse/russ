use std::{
    fmt::Display,
    io::{self, Read},
};

use crate::{die::Mortal, file::open};

pub fn read(path: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    open(path)
        .read_to_end(&mut buf)
        .or_die_with(|e| Error(path, e));
    buf
}

struct Error<'a>(&'a str, io::Error);

impl Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "! error reading `{}`: {}", self.0, self.1)
    }
}
