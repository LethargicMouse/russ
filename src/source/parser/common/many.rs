use crate::source::parser::{Parser, fail::PF};

impl<'a> Parser<'a> {
    pub fn many<T>(&mut self, parse: impl Fn(&mut Self) -> Result<T, PF>) -> Vec<T> {
        let mut res = Vec::new();
        while let Some(t) = self.maybe(&parse) {
            res.push(t);
        }
        res
    }
}
