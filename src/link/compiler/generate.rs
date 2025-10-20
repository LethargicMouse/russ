use crate::{link::program::Program, qbe::ir::IR};

pub fn generate(program: Program) -> IR {
    IRBuilder::new(program).build()
}
