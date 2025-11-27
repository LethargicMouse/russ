use std::iter::once;

use crate::{
    file::read,
    location::{BEGIN, Pos},
};

pub struct Source {
    pub name: String,
    pub code: String,
    pub poses: Vec<Pos>,
    lines: Vec<(usize, usize)>,
}

pub fn read_source(path: String) -> Source {
    let code = read(&path);
    Source {
        code: read(&path),
        lines: get_lines(&code),
        poses: get_poses(&code),
        name: path,
    }
}

fn get_poses(code: &str) -> Vec<Pos> {
    code.chars()
        .chain(once(' '))
        .scan(BEGIN, |p, c| {
            let res = Some(*p);
            if c == '\n' {
                p.line += 1;
                p.symbol = 1;
            } else {
                p.symbol += 1;
            }
            res
        })
        .collect()
}

impl Source {
    pub fn get_line(&self, index: usize) -> &str {
        let (start, len) = self.lines[index];
        &self.code[start..start + len]
    }
}

fn get_lines(code: &str) -> Vec<(usize, usize)> {
    code.lines()
        .map(|l| l.len())
        .scan(0, |s, l| {
            let res = Some((*s, l));
            *s += l;
            res
        })
        .collect()
}
