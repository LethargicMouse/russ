use std::fmt::Display;

use crate::pretty::repeat::Repeat;

pub trait EndsWithNewline {
    fn ends_with_newline_or_empty(&self) -> bool;
}

impl EndsWithNewline for &str {
    fn ends_with_newline_or_empty(&self) -> bool {
        self.is_empty() || self.ends_with("\n")
    }
}

pub struct Block<T>(pub &'static str, pub T);

impl<T: Display + EndsWithNewline> Display for Block<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const LINE_LEN: u16 = 40;
        const PADDING: u16 = 4;
        const REST: u16 = LINE_LEN - PADDING - 2;
        let rest = if REST >= self.0.len().min(u16::MAX as usize) as u16 {
            REST - self.0.len() as u16
        } else {
            0
        };
        write!(
            f,
            "\n{} {} {}\n{}{}{}",
            Repeat(PADDING, '-'),
            self.0,
            Repeat(rest, '-'),
            self.1,
            if self.1.ends_with_newline_or_empty() {
                ""
            } else {
                "\n"
            },
            Repeat(LINE_LEN, '-')
        )
    }
}
