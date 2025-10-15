// provides methods of operating with files

use std::{fs::File, io::Read};

use crate::death::OrDie;

pub fn read(path: &str) -> String {
    let mut buf = String::new();
    File::open(path).or_die().read_to_string(&mut buf).or_die();
    buf
}
