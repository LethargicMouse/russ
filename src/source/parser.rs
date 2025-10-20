pub mod common;
pub mod error;
pub mod fail;

use std::collections::HashSet;

use crate::{
    death::OrDie,
    source::{Source, parser::fail::PF},
};

pub struct Parser<'a> {
    pub source: &'a Source,
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

    pub fn parse<T>(mut self, f: impl Fn(&mut Self) -> Result<T, PF>) -> T {
        f(&mut self).map_err(|_| self.error()).or_die()
    }
}
