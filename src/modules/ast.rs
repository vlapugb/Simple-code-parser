#[derive(Debug)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Num(i8),
    Var(String),
}
#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Assign(String, Expr),
}
