use crate::{
    link::{ast::r#type::Type, compiler::generate::Generate},
    qbe::ir::r#type::AbiType,
};

impl<'a, 'b> Generate<'a, 'b> {
    pub fn r#type(&self, r#type: Type) -> AbiType {
        match r#type {
            Type::Unit => AbiType::WORD,
        }
    }
}
