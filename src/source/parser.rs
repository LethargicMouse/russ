pub mod common;
mod error;

use std::{cmp::Ordering, collections::HashSet};

use crate::{
    death::OrDie,
    source::{Source, parser::error::Error, pos::Pos, view::View},
};

pub struct Parser<'a> {
    source: &'a Source,
    cursor: usize,
    err_cursor: usize,
    err_msgs: HashSet<&'static str>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a Source) -> Self {
        Self {
            source,
            cursor: 0,
            err_cursor: 0,
            err_msgs: HashSet::new(),
        }
    }

    pub fn parse<T>(mut self, f: impl Fn(&mut Self) -> Result<T, ()>) -> T {
        f(&mut self).map_err(|_| self.error()).or_die()
    }

    fn error(self) -> Error<'a> {
        let view = self.view_at(self.source.poses[self.err_cursor]);
        let msgs = self.err_msgs;
        Error { view, msgs }
    }

    fn view_at(&self, pos: Pos) -> View<'a> {
        self.view(pos, pos.next(' '))
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

    pub fn eof(&mut self) -> Result<(), ()> {
        self.skip_spaces();
        if self.cursor == self.source.code.len() {
            Ok(())
        } else {
            self.fail("end of file")
        }
    }

    fn fail<T>(&mut self, msg: &'static str) -> Result<T, ()> {
        match self.cursor.cmp(&self.err_cursor) {
            Ordering::Less => {}
            Ordering::Equal => {
                self.err_msgs.insert(msg);
            }
            Ordering::Greater => {
                self.err_cursor = self.cursor;
                self.err_msgs.clear();
                self.err_msgs.insert(msg);
            }
        }
        Err(())
    }

    fn skip_spaces(&mut self) {
        self.cursor += self.source.code[self.cursor..]
            .chars()
            .take_while(|c| c.is_whitespace())
            .count();
    }
}
