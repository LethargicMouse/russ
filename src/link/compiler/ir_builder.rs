use crate::{link::program::Program, qbe::ir::IR};

pub struct IRBuilder<'a, 'b> {
    #[allow(dead_code)]
    program: &'b Program<'a>,
    result: IR,
}

impl<'a, 'b> IRBuilder<'a, 'b> {
    pub fn new(program: &'b Program<'a>) -> Self {
        let result = IR::empty();
        Self { program, result }
    }

    pub fn build(self) -> IR {
        self.result
    }
}
