use std::fmt::Display;

use crate::qbe::ir::r#type::AbiType;

pub struct Arg {
    name: String,
    r#type: AbiType,
}

impl Display for Arg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.r#type, self.name)
    }
}
