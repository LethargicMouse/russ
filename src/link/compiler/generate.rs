use crate::{
    link::{compiler::ir_builder::IRBuilder, program::Program},
    qbe::ir::IR,
};

pub fn generate(program: &Program) -> IR {
    IRBuilder::new(program).build()
}
