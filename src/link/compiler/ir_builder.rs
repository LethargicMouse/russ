use crate::{link::program::Program, qbe::ir::IR};

pub struct IRBuilder<'a, 'b> {
    program: &'b Program<'a>,
}

impl<'a, 'b> IRBuilder<'a, 'b> {
    pub fn new(program: &'b Program<'a>) -> Self {
        todo!()
    }

    pub fn build(&mut self) -> IR {
        todo!()
    }
}
