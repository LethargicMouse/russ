use crate::{
    link::ast::expr::block::Block,
    source::parser::{Parser, fail::PF},
};

pub fn block<'a>(p: &mut Parser<'a>) -> Result<Block, PF> {
    p.expect("{")?;
    p.expect("}")?;
    Ok(Block {})
}
