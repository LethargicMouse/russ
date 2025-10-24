mod stmt;
pub use stmt::stmt;

use crate::{
    link::ast::expr::Expr,
    source::parser::{Parser, fail::PF},
};

pub fn expr<'a>(p: &mut Parser<'a>) -> Result<Expr, PF> {
    p.expect("()")?;
    Ok(Expr::Unit)
}
