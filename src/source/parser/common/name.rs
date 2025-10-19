use crate::source::parser::{
    Parser,
    common::name::char::{is_name_char, is_name_first_char},
};

pub mod char;

pub fn name<'a>(p: &mut Parser<'a>) -> Result<&'a str, ()> {
    p.skip_spaces();
    let rest = &p.source.code[p.cursor..];
    let res = &rest[..rest.chars().take_while(|c| is_name_char(*c)).count()];
    if !res.is_empty() && is_name_first_char(res.chars().next().unwrap()) {
        p.cursor += res.len();
        Ok(res)
    } else {
        p.fail("name")
    }
}
