use crate::{
    link::{ast::r#type::Type, compiler::generate::Generate},
    qbe::ir,
};

impl<'a, 'b> Generate<'a, 'b> {
    pub fn r#type(&self, r#type: Type) -> ir::AbiType {
        match r#type {
            Type::Unit => ir::AbiType::WORD,
        }
    }
}
