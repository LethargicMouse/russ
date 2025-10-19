use crate::{
    link::ast::{
        expr::block::parse::block,
        fun::{Fun, header::parse::header},
    },
    source::parser::Parser,
};

pub fn fun<'a>(p: &mut Parser<'a>) -> Result<Fun, ()> {
    let _ = header(p)?;
    let _ = block(p)?;
    Ok(Fun {})
}
