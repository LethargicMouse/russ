use std::fmt::Display;

pub struct Repeat<T>(pub T, pub u32);

impl<T: Display> Display for Repeat<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.1 {
            write!(f, "{}", self.0)?;
        }
        Ok(())
    }
}
