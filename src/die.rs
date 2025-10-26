pub mod either;

use std::{fmt::Display, process::exit};

pub fn die<T>(err: impl Display) -> T {
    eprintln!("{err}");
    exit(1)
}
