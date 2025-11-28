pub struct Ast {}

pub enum Expr {
    Unit,
    Call(Call),
    Int(i64),
}

pub struct Call {}
