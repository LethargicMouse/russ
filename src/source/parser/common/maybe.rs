use crate::source::parser::{Parser, fail::PF};

impl<'a> Parser<'a> {
    pub fn maybe<T>(&mut self, parse: impl Fn(&mut Self) -> Result<T, PF>) -> Option<T> {
        let cursor = self.cursor;
        parse(self).inspect_err(|_| self.cursor = cursor).ok()
    }
}
