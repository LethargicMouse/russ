// provides type of QBE intermediate representation

use std::{fmt::Display, fs::File, io::Write};

use crate::death::OrDie;

pub struct IR {}

impl IR {
    pub fn dump(self) {
        let mut out = File::create("out.qbe").or_die();
        write!(out, "{self}").or_die()
    }
}

impl Display for IR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        Ok(())
    }
}
