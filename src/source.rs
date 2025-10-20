pub mod parser;
mod pos;
mod posify;
pub mod view;

use crate::{
    file,
    source::{pos::Pos, posify::posify},
};

pub struct Source {
    pub name: String,
    code: String,
    lines: Vec<String>,
    poses: Vec<Pos>,
}

impl Source {
    pub fn read(path: String) -> Self {
        let code = file::read(&path);
        Self::new(path, code)
    }

    pub fn new(name: String, code: String) -> Self {
        Self {
            lines: code.lines().map(String::from).collect(),
            poses: posify(&code),
            code,
            name,
        }
    }
}
