use crate::{
    link::{
        ast::fun::Fun,
        compiler::generate::{Generate, stmts::GenStmts},
    },
    qbe::ir,
};

impl<'a, 'b> Generate<'a, 'b> {
    pub fn fun(&mut self, fun: &Fun) {
        self.ir.funs.push(ir::Fun {
            ret_type: self.r#type(fun.header.ret_type),
            name: fun.name().to_owned(),
            args: [].into(),
            stmts: GenStmts::new(self).run(&fun.body),
        });
    }
}
