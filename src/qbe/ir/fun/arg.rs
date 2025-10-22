use std::fmt::Display;

use crate::qbe::ir::r#type::Abi;

pub struct Arg {
    name: String,
    r#type: Abi,
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.r#type, self.name)
    }
}
