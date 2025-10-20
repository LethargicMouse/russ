use std::fmt::Display;

use crate::source::view::View;

pub struct AlreadyDeclared<'a> {
    pub view: View<'a>,
    pub kind: &'static str,
    pub name: &'a str,
    pub old_view: View<'a>,
}

impl Display for AlreadyDeclared<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "! error {}\n--! {} `{}` is already declared {}",
            self.view, self.kind, self.name, self.old_view
        )
    }
}
