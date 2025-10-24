use crate::link::ast::fun::header::Header;

mod header;
mod name;
pub mod parse;
mod view;

#[derive(Debug)]
pub struct Fun<'a> {
    pub header: Header<'a>,
}
