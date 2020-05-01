use crate::expr::Expr;
use crate::stmt::Stmt;
use std::io::{self, Write};

#[derive(Debug)]
pub enum EvalError {
    TypeError(String),
    NotInEnv(String),
}

pub fn eval(ast: Vec<Stmt>) -> Result<(), EvalError> {
    let mut env = Env::EmptyEnv;
    for statment in ast {
        // println!("{:?}", statment);
        match statment {
            Stmt::ValDef(name, e) => match eval_expr(e, &env) {
                Ok(val) => env = Env::ExtendVal(name, val, Box::new(env.clone())),
                Err(e) => return Err(e),
            },
            Stmt::FunDef(fn_name, params, e) => {
                env = Env::ExtendFn(fn_name, params.get(0).unwrap().clone(), e, Box::new(env.clone()))
                // let closure = Val::Closure()
                // for param in params {
                //     let tmp = Val::Closure(fn_name.clone(), param, e.clone(), env.clone());
                // }
            }
            Stmt::Print(e) => match eval_expr(e, &env) {
                Ok(val) => {
                    print!("{:?}", val);
                    io::stdout().flush().unwrap();
                }
                Err(e) => return Err(e),
            },
            Stmt::Println(e) => match eval_expr(e, &env) {
                Ok(val) => {
                    println!("{:?}", val);
                }
                Err(e) => return Err(e),
            },
        }
    }
    // println!("Done");
    Ok(())
}

fn val_to_number(val: Val) -> Result<i32, EvalError> {
    match val {
        Val::IntVal(x) => Ok(x),
        _ => Err(EvalError::TypeError(format!(
            "Expected number got: {:?}",
            val
        ))),
    }
}
fn val_to_bool(val: Val) -> Result<bool, EvalError> {
    match val {
        Val::BoolVal(x) => Ok(x),
        _ => Err(EvalError::TypeError(format!(
            "Expected bool got: {:?}",
            val
        ))),
    }
}

fn eval_expr(e: Expr, env: &Env) -> Result<Val, EvalError> {
    let apply_arth2 = |e1: Expr, e2: Expr, f: fn(i32, i32) -> i32| -> Result<i32, EvalError> {
        match (eval_expr(e1, env), eval_expr(e2, env)) {
            (Ok(evaled1), Ok(evaled2)) => match (val_to_number(evaled1), val_to_number(evaled2)) {
                (Ok(v1), Ok(v2)) => Ok(f(v1, v2)),
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            },
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    };
    let apply_arth1 = |e1: Expr, f: fn(i32) -> i32| -> Result<i32, EvalError> {
        match eval_expr(e1, env) {
            Ok(evaled1) => match val_to_number(evaled1) {
                Ok(v1) => Ok(f(v1)),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    };
    let apply_comp_ordered = |e1: Expr,
                              e2: Expr,
                              f: fn(i32, i32) -> bool|
     -> Result<bool, EvalError> {
        match (eval_expr(e1, env), eval_expr(e2, env)) {
            (Ok(evaled1), Ok(evaled2)) => match (val_to_number(evaled1), val_to_number(evaled2)) {
                (Ok(v1), Ok(v2)) => Ok(f(v1, v2)),
                (Err(e), _) => Err(e),
                (_, Err(e)) => Err(e),
            },
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        }
    };
    let apply_logical1 = |e1: Expr, f: fn(bool) -> bool| -> Result<bool, EvalError> {
        match eval_expr(e1, env) {
            Ok(evaled1) => match val_to_bool(evaled1) {
                Ok(v1) => Ok(f(v1)),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    };
    match e {
        Expr::Ident(x) => match look_up_env(env, x.clone()) {
            Some(v) => Ok(v),
            None => Err(EvalError::NotInEnv(format!("{:?} not found in env", x))),
        },
        Expr::ConstInt(x) => Ok(Val::IntVal(x)),
        Expr::ConstBool(x) => Ok(Val::BoolVal(x)),
        Expr::Equal(e1, e2) => match apply_comp_ordered(*e1, *e2, |v1, v2| v1 == v2) {
            Ok(x) => Ok(Val::BoolVal(x)),
            Err(e) => Err(e),
        },
        Expr::NotEqual(e1, e2) => match apply_comp_ordered(*e1, *e2, |v1, v2| v1 != v2) {
            Ok(x) => Ok(Val::BoolVal(x)),
            Err(e) => Err(e),
        },
        Expr::Greater(e1, e2) => match apply_comp_ordered(*e1, *e2, |v1, v2| v1 > v2) {
            Ok(x) => Ok(Val::BoolVal(x)),
            Err(e) => Err(e),
        },
        Expr::GreaterEqual(e1, e2) => match apply_comp_ordered(*e1, *e2, |v1, v2| v1 >= v2) {
            Ok(x) => Ok(Val::BoolVal(x)),
            Err(e) => Err(e),
        },
        Expr::Less(e1, e2) => match apply_comp_ordered(*e1, *e2, |v1, v2| v1 < v2) {
            Ok(x) => Ok(Val::BoolVal(x)),
            Err(e) => Err(e),
        },
        Expr::LessEqual(e1, e2) => match apply_comp_ordered(*e1, *e2, |v1, v2| v1 <= v2) {
            Ok(x) => Ok(Val::BoolVal(x)),
            Err(e) => Err(e),
        },
        Expr::Minus(e1, e2) => match apply_arth2(*e1, *e2, |v1, v2| v1 - v2) {
            Ok(x) => Ok(Val::IntVal(x)),
            Err(e) => Err(e),
        },
        Expr::Plus(e1, e2) => match apply_arth2(*e1, *e2, |v1, v2| v1 + v2) {
            Ok(x) => Ok(Val::IntVal(x)),
            Err(e) => Err(e),
        },
        Expr::Div(e1, e2) => match apply_arth2(*e1, *e2, |v1, v2| v1 / v2) {
            Ok(x) => Ok(Val::IntVal(x)),
            Err(e) => Err(e),
        },
        Expr::Mult(e1, e2) => match apply_arth2(*e1, *e2, |v1, v2| v1 * v2) {
            Ok(x) => Ok(Val::IntVal(x)),
            Err(e) => Err(e),
        },
        Expr::Not(e1) => match apply_logical1(*e1, |v| !v) {
            Ok(x) => Ok(Val::BoolVal(x)),
            Err(e) => Err(e),
        },
        Expr::Neg(e1) => match apply_arth1(*e1, |v| -v) {
            Ok(x) => Ok(Val::IntVal(x)),
            Err(e) => Err(e),
        },
        Expr::Grouping(e1) => eval_expr(*e1, env),
        Expr::IfElse(e1, e2, e3) => match eval_expr(*e1, env) {
            Ok(v) => match v {
                Val::BoolVal(true) => eval_expr(*e2, env),
                Val::BoolVal(false) => eval_expr(*e3, env),
                _ => Err(EvalError::TypeError(format!(
                    "If Else require bool got: {:?}",
                    v
                ))),
            },
            Err(e) => Err(e),
        },
        Expr::FunCall(e1, e2) => match (eval_expr(*e1, env), eval_expr(*e2, env)) {
            (Ok(v1), Ok(v2)) => match v1 {
                Val::Closure(x, fn_expr, pi) => eval_expr(fn_expr, &Env::ExtendVal(x, v2, pi)),
                _ => Err(EvalError::TypeError(format!(
                    "Can not call {:?} as a function",
                    v1
                ))),
            },
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
        },
    }
}

#[derive(Debug, Clone)]
enum Env {
    EmptyEnv,
    ExtendVal(String, Val, Box<Env>),
    ExtendFn(String, String, Expr, Box<Env>),
}

fn look_up_env(sigma: &Env, x: String) -> Option<Val> {
    match sigma {
        Env::EmptyEnv => None,
        Env::ExtendVal(ident, value, pi) => {
            if &x == ident {
                Some(value.clone())
            } else {
                look_up_env(pi, x)
            }
        }
        Env::ExtendFn(f, param, fn_body, pi) => {
            if f == &x {
                Some(Val::Closure(
                    param.clone(),
                    fn_body.clone(),
                    Box::new(sigma.clone()),
                ))
            } else {
                look_up_env(pi, x)
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Val {
    IntVal(i32),
    BoolVal(bool),
    Closure(String, Expr, Box<Env>),
}
