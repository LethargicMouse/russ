mod empty;
mod fun;
pub use fun::Fun;
mod r#type;
pub use r#type::AbiType;
mod stmt;
pub use stmt::Stmt;

use std::{fmt::Display, fs::File, io::Write};

use crate::death::OrDie;

pub struct IR {
    pub funs: Vec<Fun>,
}

impl IR {
    pub fn dump(self) {
        let mut out = File::create("out.qbe").or_die();
        write!(out, "{self}").or_die()
    }
}

impl Display for IR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "export")?;
        for fun in &self.funs {
            write!(f, "{fun}")?;
        }
        Ok(())
    }
}
