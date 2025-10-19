use std::{iter::once, mem::replace};

use crate::source::pos::Pos;

pub fn posify(code: &str) -> Vec<Pos> {
    code.chars()
        .chain(once(' '))
        .scan(Pos::START, |p, c| Some(replace(p, p.next(c))))
        .collect()
}
