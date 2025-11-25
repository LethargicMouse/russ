use std::{fmt::Display, process::Command};

use crate::{
    die::{Mortal, die},
    display::block::Block,
    process::Error,
};

pub fn call(path: &str, args: &[&str]) {
    let output = Command::new(path)
        .args(args)
        .output()
        .or_die_with(|e| Error(path, e));
    if !output.status.success() {
        die(Fail(path, output.stdout, output.stderr))
    }
}

struct Fail<'a>(&'a str, Vec<u8>, Vec<u8>);

impl Display for Fail<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stdout = str::from_utf8(&self.1).unwrap();
        let stderr = str::from_utf8(&self.2).unwrap();
        write!(
            f,
            "! error running `{}`:\n{}\n{}",
            self.0,
            Block("stdout", stdout),
            Block("stderr", stderr)
        )
    }
}
