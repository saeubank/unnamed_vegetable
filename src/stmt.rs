use crate::expr::Expr;
#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Println(Expr),
    Let(String, Expr),
    IfElse(Expr, Expr),
    Nothing(Expr),
}