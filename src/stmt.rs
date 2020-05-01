use crate::expr::Expr;
#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Println(Expr),
    ValDef(String, Expr),
    FunDef(String, Vec<String>, Expr),
}
