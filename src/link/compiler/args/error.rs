use std::fmt::Display;

pub struct ExpectedPath;

impl Display for ExpectedPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "expected path")
    }
}

pub enum Error {
    EP(ExpectedPath),
}

impl From<ExpectedPath> for Error {
    fn from(v: ExpectedPath) -> Self {
        Self::EP(v)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "! error in args: ")?;
        match self {
            Error::EP(expected_path) => write!(f, "{expected_path}"),
        }
    }
}
