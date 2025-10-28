use std::fmt::Display;

pub struct Repeat(pub u16, pub char);

impl Display for Repeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.0 {
            write!(f, "{}", self.1)?;
        }
        Ok(())
    }
}
