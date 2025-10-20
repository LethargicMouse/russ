use crate::source::parser::{Parser, fail::PF};

impl<'a> Parser<'a> {
    pub fn expect(&mut self, s: &'static str) -> Result<(), PF> {
        self.skip_spaces();
        if self.source.code[self.cursor..].starts_with(s) {
            self.cursor += s.len();
            Ok(())
        } else {
            self.fail(s)
        }
    }
}
