pub mod analyse;

pub struct Program<'a> {
    name: &'a str,
}

impl<'a> Program<'a> {
    pub fn empty(name: &'a str) -> Self {
        Self { name }
    }
}
