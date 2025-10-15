// provides type of command fail

use std::fmt::Display;

use crate::pretty::Block;

pub struct Fail<'a> {
    pub name: &'a str,
    pub code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl Display for Fail<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "! error running command `{}`:\n--! failed with exit code {}\n{}{}",
            self.name,
            self.code,
            Block("stdout", std::str::from_utf8(&self.stdout).unwrap()),
            Block("stderr", std::str::from_utf8(&self.stderr).unwrap())
        )
    }
}
