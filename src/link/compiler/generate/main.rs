use crate::link::compiler::generate::Generate;

impl<'a, 'b> Generate<'a, 'b> {
    pub fn main(&mut self) {
        let main = &self.program.funs["main"];
        self.fun(main);
    }
}
