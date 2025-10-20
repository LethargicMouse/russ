use crate::source::parser::Parser;

impl<'a> Parser<'a> {
    pub fn skip_spaces(&mut self) {
        self.cursor += self.source.code[self.cursor..]
            .chars()
            .take_while(|c| c.is_whitespace())
            .count();
    }
}
