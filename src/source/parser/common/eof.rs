use crate::source::parser::{Parser, fail::PF};

pub fn eof<'a>(p: &mut Parser<'a>) -> Result<(), PF> {
    p.skip_spaces();
    if p.cursor == p.source.code.len() {
        Ok(())
    } else {
        p.fail("end of file")
    }
}
