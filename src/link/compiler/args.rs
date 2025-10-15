// provides type of arguments for Link compiler
mod error;

use std::env::args;

use crate::{
    death::OrDie,
    link::compiler::args::error::{Error, ExpectedPath},
};

pub struct Args {
    pub path: String,
}

impl Args {
    pub fn get() -> Self {
        let mut raw = args();
        raw.next();
        let path = raw.next().ok_or_else(|| Error::from(ExpectedPath)).or_die();
        Self { path }
    }
}
