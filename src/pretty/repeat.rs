use std::fmt::Display;

pub struct Repeat<T>(pub u16, pub T);

impl<T: Display> Display for Repeat<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.0 {
            write!(f, "{}", self.1)?;
        }
        Ok(())
    }
}
