use std::fmt::Display;

pub struct Line<'a>(pub u16, pub &'a str);

impl Display for Line<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{:4} | {}", self.0, self.1)
    }
}
