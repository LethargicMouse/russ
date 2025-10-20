use crate::source::{
    parser::{Parser, fail::PF},
    view::viewed::Viewed,
};

impl<'a> Parser<'a> {
    pub fn viewed<T>(
        &mut self,
        parse: impl Fn(&mut Self) -> Result<T, PF>,
    ) -> Result<Viewed<'a, T>, PF> {
        let start = self.source.poses[self.cursor];
        let un = parse(self)?;
        let end = self.source.poses[self.cursor];
        let view = self.source.view(start, end);
        Ok(Viewed { view, un })
    }
}
