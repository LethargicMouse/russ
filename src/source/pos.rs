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
    pub const START: Pos = Pos { line: 1, symbol: 1 };
}
