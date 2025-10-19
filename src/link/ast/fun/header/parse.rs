use crate::{
    link::ast::fun::header::Header,
    source::parser::{Parser, common::name},
};

pub fn header<'a>(p: &mut Parser<'a>) -> Result<Header, ()> {
    p.expect("fn")?;
    let _ = name(p)?;
    p.expect("(")?;
    p.expect(")")?;
    Ok(Header {})
}
