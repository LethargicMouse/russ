use std::fmt::Display;

use crate::pretty::repeat::Repeat;

pub struct Block<T>(pub &'static str, pub T);

impl<T: Display> Display for Block<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const LINE_LEN: u16 = 32;
        const PADDING: u16 = 8;
        write!(
            f,
            "\n{} {} {}\n{}\n{}",
            Repeat(PADDING, '-'),
            self.0,
            Repeat(LINE_LEN - PADDING - 2, '-'),
            self.1,
            Repeat(LINE_LEN, '-')
        )
    }
}
