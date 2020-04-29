use crate::token::Token;

//             Token::Let => {scan till end of decloration}
//             Token::If => {scan till Else ends?}
//             Token::Print
//             Token::Println
// use crate::token::Token;

pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParseError> {
    // turn newline+ into newline?
    // turn tabs into {}?
    // turn fn ... -> Expr to fn ... -> Result<Expr, error>
    equality(&tokens)
}

// need to figure out how to implement this given recursive code
// perhaps use how many tokens left to figure out
// fn get_curr_line(tokens: &Vec<Token>, i: usize) -> usize {
//     let mut count = 0;
//     for i in 0..i {
//         let token = tokens.get(i);
//         match token {
//             Some(Token::NewLine) => count += 1,
//             _ => {}
//         }
//     }
//     count
// }

#[derive(Debug)]
pub enum Expr {
    // Ident(String),
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

#[derive(Debug)]
pub enum ParseError {
    Error(usize, String),
}

// need to figure out how to handle when tokens.length() == 0 (assuming this needs to be handled)
// match: comp ((== | !=) comp)*
fn equality(tokens: &[Token]) -> Result<Expr, ParseError> {
    match comparison(&tokens) {
        Ok(expr) => equality_helper(expr, &tokens[1..]),
        e @ Err(_) => e,
    }
}

fn equality_helper(expr: Expr, tokens: &[Token]) -> Result<Expr, ParseError> {
    match tokens.get(0) {
        Some(Token::EqualEqual) => match comparison(&tokens[1..]) {
            Ok(right) => {
                equality_helper(Expr::Equal(Box::new(expr), Box::new(right)), &tokens[2..])
            }
            e @ Err(_) => e,
        },
        Some(Token::BangEqual) => match comparison(&tokens[1..]) {
            Ok(right) => equality_helper(
                Expr::NotEqual(Box::new(expr), Box::new(right)),
                &tokens[2..],
            ),
            e @ Err(_) => e,
        },
        _ => Ok(expr),
    }
}

// match: add ((> | >= | < | <=) add)*
fn comparison(tokens: &[Token]) -> Result<Expr, ParseError> {
    match addition(&tokens) {
        Ok(expr) => comparison_helper(expr, &tokens[1..]),
        e @ Err(_) => e,
    }
}

fn comparison_helper(expr: Expr, tokens: &[Token]) -> Result<Expr, ParseError> {
    match tokens.get(0) {
        Some(Token::Greater) => match addition(&tokens[1..]) {
            Ok(right) => {
                comparison_helper(Expr::Greater(Box::new(expr), Box::new(right)), &tokens[2..])
            }
            e @ Err(_) => e,
        },
        Some(Token::GreaterEqual) => match addition(&tokens[1..]) {
            Ok(right) => comparison_helper(
                Expr::GreaterEqual(Box::new(expr), Box::new(right)),
                &tokens[2..],
            ),
            e @ Err(_) => e,
        },
        Some(Token::Less) => match addition(&tokens[1..]) {
            Ok(right) => {
                comparison_helper(Expr::Less(Box::new(expr), Box::new(right)), &tokens[2..])
            }
            e @ Err(_) => e,
        },
        Some(Token::LessEqual) => match addition(&tokens[1..]) {
            Ok(right) => comparison_helper(
                Expr::LessEqual(Box::new(expr), Box::new(right)),
                &tokens[2..],
            ),
            e @ Err(_) => e,
        },
        _ => Ok(expr),
    }
}

// match: mult ((- | +) mult)*
fn addition(tokens: &[Token]) -> Result<Expr, ParseError> {
    match multiplication(&tokens) {
        Ok(expr) => addition_helper(expr, &tokens[1..]),
        e @ Err(_) => e,
    }
}

fn addition_helper(expr: Expr, tokens: &[Token]) -> Result<Expr, ParseError> {
    match tokens.get(0) {
        Some(Token::Minus) => match multiplication(&tokens[1..]) {
            Ok(right) => {
                addition_helper(Expr::Minus(Box::new(expr), Box::new(right)), &tokens[2..])
            }
            e @ Err(_) => e,
        },
        Some(Token::Plus) => match multiplication(&tokens[1..]) {
            Ok(right) => addition_helper(Expr::Plus(Box::new(expr), Box::new(right)), &tokens[2..]),
            e @ Err(_) => e,
        },
        _ => Ok(expr),
    }
}

// match: unary ((/ | *) unary)*
fn multiplication(tokens: &[Token]) -> Result<Expr, ParseError> {
    match unary(&tokens) {
        Ok(expr) => multiplication_helper(expr, &tokens[1..]),
        e @ Err(_) => e,
    }
}

fn multiplication_helper(expr: Expr, tokens: &[Token]) -> Result<Expr, ParseError> {
    match tokens.get(0) {
        Some(Token::Slash) => match unary(&tokens[1..]) {
            Ok(right) => {
                multiplication_helper(Expr::Div(Box::new(expr), Box::new(right)), &tokens[2..])
            }
            e @ Err(_) => e,
        },
        Some(Token::Star) => match unary(&tokens[1..]) {
            Ok(right) => {
                multiplication_helper(Expr::Mult(Box::new(expr), Box::new(right)), &tokens[2..])
            }
            e @ Err(_) => e,
        },
        _ => Ok(expr),
    }
}

// match: ((! | -)*unary)
fn unary(tokens: &[Token]) -> Result<Expr, ParseError> {
    let curr_token = tokens.get(0);
    match curr_token {
        Some(Token::Bang) => match unary(&tokens[1..]) {
            Ok(right) => Ok(Expr::Not(Box::new(right))),
            e @ Err(_) => e,
        },
        Some(Token::Minus) => match unary(&tokens[1..]) {
            Ok(right) => Ok(Expr::Neg(Box::new(right))),
            e @ Err(_) => e,
        },
        _ => primary(&tokens),
    }
}

fn primary(tokens: &[Token]) -> Result<Expr, ParseError> {
    let curr_token = tokens.get(0);
    match curr_token {
        Some(Token::True) => Ok(Expr::ConstBool(true)),
        Some(Token::False) => Ok(Expr::ConstBool(false)),
        Some(Token::Number(num)) => Ok(Expr::ConstInt(num.clone())),
        Some(t) => Err(ParseError::Error(
            // get_curr_line(&tokens, i),
            0,
            format!("{:?}", t),
        )),
        None => Err(ParseError::Error(
            // get_curr_line(&tokens, i),
            0,
            "Got none in parsing".to_string(),
        )),
    }
}
