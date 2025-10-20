use crate::{
    link::ast::fun::header::Header,
    source::parser::{Parser, common::name, fail::PF},
};

pub fn header<'a>(p: &mut Parser<'a>) -> Result<Header<'a>, PF> {
    p.expect("fn")?;
    let name = p.viewed(name)?;
    p.expect("(")?;
    p.expect(")")?;
    Ok(Header { name })
}
