use crate::stmt::Stmt;

pub enum EvalError {
    Error(String),
}

pub fn eval(ast: Vec<Stmt>) -> Result<(), EvalError> {
    Ok(())
}

struct Env {}
