use crate::{
    link::ast::expr::{
        Expr,
        block::Block,
        parse::{expr, stmt},
    },
    source::parser::{Parser, fail::PF},
};

pub fn block<'a>(p: &mut Parser<'a>) -> Result<Block, PF> {
    p.expect("{")?;
    let stmts = p.many(stmt);
    let ret = p.maybe(expr).unwrap_or(Expr::Unit);
    p.expect("}")?;
    Ok(Block { stmts, ret })
}
