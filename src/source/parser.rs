mod error;

use std::collections::HashSet;

use crate::{
    death::OrDie,
    source::{Source, parser::error::Error, pos::Pos, view::View},
};

pub struct Parser<'a> {
    source: &'a Source,
    err_pos: Pos,
    err_msgs: HashSet<&'static str>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a Source) -> Self {
        Self {
            source,
            err_pos: Pos::START,
            err_msgs: HashSet::new(),
        }
    }

    pub fn parse<T>(mut self, f: impl Fn(&mut Self) -> Result<T, ()>) -> T {
        f(&mut self).map_err(|_| self.error()).or_die()
    }

    fn error(self) -> Error<'a> {
        let view = self.view_at(self.err_pos);
        let msgs = self.err_msgs;
        Error { view, msgs }
    }

    fn view_at(&self, pos: Pos) -> View<'a> {
        let mut pos2 = pos;
        pos2.symbol += 1;
        self.view(pos, pos2)
    }

    fn view(&self, start: Pos, end: Pos) -> View<'a> {
        View {
            src_name: &self.source.name,
            lines: &self.source.lines,
            start,
            end,
        }
    }

    pub fn source_name(&self) -> &'a str {
        &self.source.name
    }
}
