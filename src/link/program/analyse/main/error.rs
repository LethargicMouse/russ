use std::fmt::Display;

pub struct NoMain<'a> {
    pub src_name: &'a str,
}

impl Display for NoMain<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "! error in {}:\n\
             --! function `main` is not declared\n\
             --@ function `main` is an entry point to the program, so it must be declared",
            self.src_name
        )
    }
}
