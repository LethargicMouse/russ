use crate::qbe::ir::IR;

impl IR {
    pub fn empty() -> Self {
        Self { funs: Vec::new() }
    }
}
