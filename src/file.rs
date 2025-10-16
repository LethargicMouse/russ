// provides methods of operating with files
mod error;

use std::{fs::File, io::Read};

use crate::{death::OrDie, file::error::OpenError};

pub fn read(path: &str) -> String {
    let mut buf = String::new();
    File::open(path)
        .map_err(|e| OpenError(path, e))
        .or_die()
        .read_to_string(&mut buf)
        .or_die();
    buf
}
