use std::fmt::Display;

pub struct Error<O, P, I>(pub O, pub P, pub I);

impl<O: Display, P: Display, I: Display> Display for Error<O, P, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "! failed to {} `{}`: {}", self.0, self.1, self.2)
    }
}
