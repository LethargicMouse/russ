pub struct Ast {
    pub expr: Expr,
}

pub enum Expr {
    Unit,
    Call(Call),
    Int(i32),
}

pub struct Call {}
