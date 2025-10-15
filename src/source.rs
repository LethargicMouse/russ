use crate::file;

// provides type of general programming language source code (with metadata like source name)
pub struct Source {}

impl Source {
    pub fn read(path: String) -> Self {
        let code = file::read(&path);
        Self::new(path, code)
    }

    pub fn new(name: String, code: String) -> Self {
        let _ = name;
        let _ = code;
        Self {}
    }
}
