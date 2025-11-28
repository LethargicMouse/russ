pub struct Ast<'a> {
    pub expr: Expr<'a>,
}

pub enum Expr<'a> {
    Unit,
    Call(Call<'a>),
    Int(i32),
}

pub struct Call<'a> {
    pub arg: Box<Expr<'a>>,
    pub name: &'a str,
}
