pub mod get;
mod line;
mod underline;
pub mod viewed;

use std::fmt::Display;

use crate::source::{
    pos::Pos,
    view::{line::Line, underline::Underline},
};

#[derive(Clone, Copy)]
pub struct View<'a> {
    pub src_name: &'a str,
    pub lines: &'a [String],
    pub start: Pos,
    pub end: Pos,
}

impl Display for View<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "in {} at {}:\n     |{}",
            self.src_name,
            self.start,
            Line(self.start.line, &self.lines[self.start.line as usize - 1])
        )?;
        if self.start.line == self.end.line {
            return write!(f, "{}", Underline(self.start.symbol, self.end.symbol));
        }
        write!(
            f,
            "{}",
            Underline(
                self.start.symbol,
                self.lines[self.start.line as usize - 1].len() as u16
            )
        )?;
        for i in self.start.line..self.end.line {
            write!(f, "{}", Line(i, &self.lines[i as usize]))?;
        }
        write!(f, "{}", Underline(1, self.end.symbol))
    }
}
