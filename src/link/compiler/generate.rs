mod fun;
mod main;
mod r#type;

use crate::{link::program::Program, qbe::ir::IR};

pub fn generate(program: &Program) -> IR {
    Generate::new(program).run()
}

struct Generate<'a, 'b> {
    program: &'b Program<'a>,
    result: IR,
}

impl<'a, 'b> Generate<'a, 'b> {
    fn new(program: &'b Program<'a>) -> Self {
        let result = IR::empty();
        Self { program, result }
    }

    fn run(mut self) -> IR {
        self.main();
        self.result
    }
}
