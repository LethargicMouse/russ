use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Pos {
    pub line: u16,
    pub symbol: u16,
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.symbol)
    }
}

impl Pos {
    pub const START: Self = Self { line: 1, symbol: 1 };

    pub fn next(mut self, c: char) -> Self {
        if c == '\n' {
            self.line += 1;
            self.symbol = 0;
        } else {
            self.symbol += 1;
        }
        self
    }
}
