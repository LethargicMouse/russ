use std::fmt::Display;

pub struct Underline(pub u16, pub u16);

impl Display for Underline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n     |")?;
        for _ in 0..self.0 {
            write!(f, " ")?;
        }
        for _ in self.0..self.1 {
            write!(f, "`")?;
        }
        Ok(())
    }
}
