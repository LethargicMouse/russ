use crate::link::ast::fun::Fun;

impl<'a> Fun<'a> {
    pub fn name(&self) -> &'a str {
        self.header.name.un
    }
}
