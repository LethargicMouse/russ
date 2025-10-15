// mod command:
// provides a function to run shell commands

mod fail;

use std::process::Command;

use crate::{
    command::fail::Fail,
    death::{OrDie, die},
};

pub fn run(name: &str, args: &[&str]) {
    let output = Command::new(name).args(args).output().or_die();
    if !output.status.success() {
        die(Fail {
            name,
            code: output.status.code().unwrap(),
            stdout: output.stdout,
            stderr: output.stderr,
        })
    }
}
