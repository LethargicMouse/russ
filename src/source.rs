pub mod parser;
pub mod pos;
pub mod view;

use crate::file;

pub struct Source {
    name: String,
    lines: Vec<String>,
}

impl Source {
    pub fn read(path: String) -> Self {
        let code = file::read(&path);
        Self::new(path, code)
    }

    pub fn new(name: String, code: String) -> Self {
        let lines = code.lines().map(|l| l.to_owned()).collect();
        Self { name, lines }
    }
}
