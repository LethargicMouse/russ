use crate::source::view::viewed::Viewed;

pub mod parse;

#[derive(Debug)]
pub struct Header<'a> {
    pub name: Viewed<'a, &'a str>,
}
