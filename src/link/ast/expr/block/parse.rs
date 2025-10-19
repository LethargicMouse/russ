use crate::{link::ast::expr::block::Block, source::parser::Parser};

pub fn block<'a>(p: &mut Parser<'a>) -> Result<Block, ()> {
    p.expect("{")?;
    p.expect("}")?;
    Ok(Block {})
}
