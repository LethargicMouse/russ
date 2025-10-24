use crate::link::ast::{expr::block::Block, fun::header::Header};

mod header;
mod name;
pub mod parse;
mod view;

#[derive(Debug)]
pub struct Fun<'a> {
    pub header: Header<'a>,
    pub body: Block,
}
