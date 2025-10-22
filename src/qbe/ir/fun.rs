pub mod arg;

use std::fmt::Display;

use crate::qbe::ir::{fun::arg::Arg, r#type::Abi};

pub struct Fun {
    ret_type: Abi,
    name: String,
    args: Vec<Arg>,
}

impl Display for Fun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nfunction {} ${}(", self.ret_type, self.name)?;
        for arg in &self.args {
            write!(f, "{arg}, ")?;
        }
        Ok(())
    }
}
