use crate::{link::ast::fun::Fun, source::view::View};

impl<'a> Fun<'a> {
    pub fn view(&self) -> View<'a> {
        self.header.name.view
    }
}
