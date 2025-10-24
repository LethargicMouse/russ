use crate::{
    link::ast::expr::Expr,
    source::parser::{Parser, fail::PF},
};

pub fn stmt<'a>(p: &mut Parser<'a>) -> Result<Expr, PF> {
    todo!()
}
