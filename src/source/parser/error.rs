use std::{collections::HashSet, fmt::Display};

use crate::source::view::View;

pub struct Error<'a> {
    pub view: View<'a>,
    pub msgs: HashSet<&'static str>,
}

impl Display for Error<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "! parse error {}\n--! expected:", self.view)?;
        for msg in &self.msgs {
            write!(f, "\n    - {msg}")?;
        }
        Ok(())
    }
}
