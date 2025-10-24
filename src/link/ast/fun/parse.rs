use crate::{
    link::ast::{
        expr::block::parse::block,
        fun::{Fun, header::parse::header},
    },
    source::parser::{Parser, fail::PF},
};

pub fn fun<'a>(p: &mut Parser<'a>) -> Result<Fun<'a>, PF> {
    let header = header(p)?;
    let body = block(p)?;
    Ok(Fun { header, body })
}
