use crate::{
    link::ast::fun::header::Header,
    source::parser::{Parser, common::name, fail::PF},
};

pub fn header<'a>(p: &mut Parser<'a>) -> Result<Header, PF> {
    p.expect("fn")?;
    let _ = name(p)?;
    p.expect("(")?;
    p.expect(")")?;
    Ok(Header {})
}
