use crate::{
    link::{
        ast::{parse::parse_ast, structure::structure},
        program::analyse::analyse,
    },
    qbe::ir::IR,
    source::Source,
};

pub fn build(source: &Source) -> IR {
    let ast = parse_ast(source);
    let program = structure(ast);
    analyse(program);
    IR {}
}
