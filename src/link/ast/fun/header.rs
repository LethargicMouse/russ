use crate::{link::ast::r#type::Type, source::view::viewed::Viewed};

pub mod parse;

#[derive(Debug, Clone, Copy)]
pub struct Header<'a> {
    pub name: Viewed<'a, &'a str>,
    pub ret_type: Type,
}
