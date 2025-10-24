use crate::{
    link::{ast::fun::Fun, compiler::generate::Generate},
    qbe::ir,
};

impl<'a, 'b> Generate<'a, 'b> {
    pub fn fun(&mut self, fun: &Fun) {
        let ret_type = self.r#type(fun.header.ret_type);
        let name = fun.name().to_owned();
        let args = [].into();
        self.result.funs.push(ir::Fun {
            ret_type,
            name,
            args,
        });
    }
}
