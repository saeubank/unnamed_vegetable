use crate::token::Token;

// make parse tokens error and replace
use std::num::ParseIntError;

fn parse(tokens: Vec<Token>) -> Result<(), ParseIntError> {
    // turn newline+ into newline
    // turn tabs into {}
    Ok(())
}

fn get_curr_line() -> i32 {
    0
}

enum Expr {
    Ident(String),
    ConstInt(i32),
    ConstBool(bool),
    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    GreaterEqual(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    LessEqual(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Plus(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Neg(Box<Expr>),
}

fn equality() -> Expr {
    let mut expr = comparison();
    loop {
        match get_curr() {
            Token::EqualEqual => {
                let right = comparison();
                expr = Expr::Equal(Box::new(expr), Box::new(right));
            }
            Token::BangEqual => {
                let right = comparison();
                expr = Expr::NotEqual(Box::new(expr), Box::new(right));
            }
            _ => break,
        }
        advance();
    }
    expr
}

fn advance() {}

fn comparison() -> Expr {
    let mut expr = addition();
    loop {
        match get_curr() {
            Token::Greater => {
                let right = addition();
                expr = Expr::Greater(Box::new(expr), Box::new(right));
            }
            Token::GreaterEqual => {
                let right = addition();
                expr = Expr::GreaterEqual(Box::new(expr), Box::new(right));
            }
            Token::Less => {
                let right = addition();
                expr = Expr::Less(Box::new(expr), Box::new(right));
            }
            Token::LessEqual => {
                let right = addition();
                expr = Expr::LessEqual(Box::new(expr), Box::new(right));
            }
            _ => break,
        }
        advance();
    }
    expr
}

fn addition() -> Expr {
    let mut expr = multiplication();
    loop {
        match get_curr() {
            Token::Minus => {
                let right = multiplication();
                expr = Expr::Minus(Box::new(expr), Box::new(right));
            }
            Token::Plus => {
                let right = multiplication();
                expr = Expr::Plus(Box::new(expr), Box::new(right));
            }
            _ => break,
        }
        advance();
    }
    expr
}

fn multiplication() -> Expr {
    let mut expr = unary();
    loop {
        match get_curr() {
            Token::Slash => {
                let right = unary();
                expr = Expr::Div(Box::new(expr), Box::new(right));
            }
            Token::Star => {
                let right = unary();
                expr = Expr::Mult(Box::new(expr), Box::new(right));
            }
            _ => break,
        }
        advance();
    }
    expr
}

fn unary() -> Expr {
    match get_curr() {
        Token::Bang => {
            let right = unary();
            Expr::Not(Box::new(right))
        }
        Token::Minus => {
            let right = unary();
            Expr::Neg(Box::new(right))
        }
        _ => primary(),
    }
}

fn primary() -> Expr {
    match get_curr() {
        Token::True => Expr::ConstBool(true),
        Token::False => Expr::ConstBool(false),
        Token::Number(num) => Expr::ConstInt(num),
        _ => panic!("yo"),
    }
}

// fn get_curr(tokens: &Vec<Token>, curr: i32) -> Token {
//     tokens.get(curr)?
// }

fn get_curr() -> Token {
    Token::BangEqual
}
