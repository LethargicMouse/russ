use std::fmt::Display;

use crate::display::Repeat;

pub struct Block<'a>(pub &'static str, pub &'a [u8]);

impl Display for Block<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PAD: u16 = 4;
        const LINE_LEN: u16 = 40;
        let after = LINE_LEN - PAD - self.0.len() as u16 - 2;
        write!(
            f,
            "\n{topl} {name} {topr}\n{data}{maybeln}{bot}",
            topl = Repeat(PAD, '-'),
            name = self.0,
            topr = Repeat(after, '-'),
            data = str::from_utf8(self.1).unwrap(),
            maybeln = if self.1.is_empty() || *self.1.last().unwrap() == b'\n' {
                ""
            } else {
                "\n"
            },
            bot = Repeat(LINE_LEN, '-')
        )
    }
}
