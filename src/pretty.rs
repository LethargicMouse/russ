// provides types to display information in a pretty way

use std::fmt::Display;

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

struct Repeat<T>(u16, T);

impl<T: Display> Display for Repeat<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.0 {
            write!(f, "{}", self.1)?;
        }
        Ok(())
    }
}
