use crate::source::parser::Parser;

impl<'a> Parser<'a> {
    pub fn expect(&mut self, s: &'static str) -> Result<(), ()> {
        self.skip_spaces();
        if self.source.code[self.cursor..].starts_with(s) {
            self.cursor += s.len();
            Ok(())
        } else {
            self.fail(s)
        }
    }
}
