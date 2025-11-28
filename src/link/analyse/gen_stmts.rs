use crate::{
    link::ast::{Call, Expr},
    qbe::ir::{Stmt, Tmp},
};

pub fn gen_stmts(expr: Expr) -> Vec<Stmt> {
    let mut gen_stmts = GenStmts::new();
    gen_stmts.expr(expr);
    gen_stmts.result
}

struct GenStmts {
    result: Vec<Stmt>,
    next_tmp: Tmp,
}

impl GenStmts {
    fn new() -> Self {
        Self {
            result: Vec::new(),
            next_tmp: 1,
        }
    }

    fn expr(&mut self, expr: Expr) -> Tmp {
        match expr {
            Expr::Unit => self.unit(),
            Expr::Call(call) => self.call(call),
            Expr::Int(n) => self.int(n),
        }
    }

    fn call(&mut self, call: Call) -> Tmp {
        todo!()
    }

    fn unit(&mut self) -> Tmp {
        self.int(0)
    }

    fn int(&mut self, n: i32) -> Tmp {
        let tmp = self.next_tmp();
        self.result.push(Stmt::Copy(tmp, n));
        tmp
    }

    fn next_tmp(&mut self) -> Tmp {
        let res = self.next_tmp;
        self.next_tmp += 1;
        res
    }
}
