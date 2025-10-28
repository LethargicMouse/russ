use std::{fmt::Display, process::Output};

use crate::display::Block;

pub struct Fail<P>(pub P, pub Output);

impl<P: Display> Display for Fail<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "! command `{}` failed with exit code {}\n{}\n{}",
            self.0,
            self.1.status,
            Block("stdout", &self.1.stdout),
            Block("stdout", &self.1.stderr)
        )
    }
}
