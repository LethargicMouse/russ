mod error;
pub use error::Error;
mod fail;
use fail::Fail;

use std::{ffi::OsStr, process::Command};

use crate::die::{die, either::OrDie};

pub fn run(name: impl AsRef<OsStr>, args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let output = Command::new(name)
        .args(args)
        .output()
        .or_die_with(|e| Error("qbe", e));
    if !output.status.success() {
        die(Fail("qbe", output))
    }
}
