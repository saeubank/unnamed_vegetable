use crate::token::Token;

pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParseError> {
    // turn newline+ into newline?
    // turn tabs into {}?
    // turn fn ... -> Expr to fn ... -> Result<Expr, error>
    equality(&tokens, 0)
}

fn get_curr_line() -> i32 {
    0
}

#[derive(Debug)]
pub enum Expr {
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

pub enum ParseError {
    Error(i32, String),
}

fn equality(tokens: &Vec<Token>, i: usize) -> Result<Expr, ParseError> {
    let mut expr = match comparison(&tokens, i) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    let mut i = i;
    loop {
        let curr_token = tokens.get(i);
        match curr_token {
            Some(Token::EqualEqual) => {
                let rest = comparison(&tokens, i);
                match rest {
                    Ok(right) => expr = Expr::Equal(Box::new(expr), Box::new(right)),
                    _ => return rest,
                }
            }
            Some(Token::BangEqual) => {
                let rest = comparison(&tokens, i);
                match rest {
                    Ok(right) => expr = Expr::NotEqual(Box::new(expr), Box::new(right)),
                    _ => return rest,
                }
            }
            _ => break,
        }
        i += 1;
    }
    Ok(expr)
}

fn comparison(tokens: &Vec<Token>, i: usize) -> Result<Expr, ParseError> {
    let mut expr = match addition(&tokens, i) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    let mut i = i;
    loop {
        let curr_token = tokens.get(i);
        match curr_token {
            Some(Token::Greater) => {
                let rest = addition(&tokens, i);
                match rest {
                    Ok(right) => expr = Expr::Greater(Box::new(expr), Box::new(right)),
                    _ => return rest,
                }
            }
            Some(Token::GreaterEqual) => {
                let rest = addition(&tokens, i);
                match rest {
                    Ok(right) => expr = Expr::GreaterEqual(Box::new(expr), Box::new(right)),
                    _ => return rest,
                }
            }
            Some(Token::Less) => {
                let rest = addition(&tokens, i);
                match rest {
                    Ok(right) => expr = Expr::Less(Box::new(expr), Box::new(right)),
                    _ => return rest,
                }
            }
            Some(Token::LessEqual) => {
                let rest = addition(&tokens, i);
                match rest {
                    Ok(right) => expr = Expr::LessEqual(Box::new(expr), Box::new(right)),
                    _ => return rest,
                }
            }
            _ => break,
        }
        i += 1;
    }
    Ok(expr)
}

fn addition(tokens: &Vec<Token>, i: usize) -> Result<Expr, ParseError> {
    let mut expr = match multiplication(&tokens, i) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    let mut i = i;
    loop {
        let curr_token = tokens.get(i);
        match curr_token {
            Some(Token::Minus) => {
                let rest = multiplication(&tokens, i);
                match rest {
                    Ok(right) => expr = Expr::Minus(Box::new(expr), Box::new(right)),
                    _ => return rest,
                }
            }
            Some(Token::Plus) => {
                let rest = multiplication(&tokens, i);
                match rest {
                    Ok(right) => expr = Expr::Plus(Box::new(expr), Box::new(right)),
                    _ => return rest,
                }
            }
            _ => break,
        }
        i += 1;
    }
    Ok(expr)
}

fn multiplication(tokens: &Vec<Token>, i: usize) -> Result<Expr, ParseError> {
    let mut expr = match unary(&tokens, i) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    let mut i = i;
    loop {
        let curr_token = tokens.get(i);
        match curr_token {
            Some(Token::Slash) => {
                let rest = unary(&tokens, i);
                match rest {
                    Ok(right) => expr = Expr::Div(Box::new(expr), Box::new(right)),
                    _ => return rest,
                }
            }
            Some(Token::Star) => {
                let rest = unary(&tokens, i);
                match rest {
                    Ok(right) => expr = Expr::Mult(Box::new(expr), Box::new(right)),
                    _ => return rest,
                }
            }
            _ => break,
        }
        i += 1;
    }
    Ok(expr)
}

fn unary(tokens: &Vec<Token>, i: usize) -> Result<Expr, ParseError> {
    let curr_token = tokens.get(i);
    match curr_token {
        Some(Token::Bang) => {
            let rest = unary(&tokens, i);
            match rest {
                Ok(right) => Ok(Expr::Not(Box::new(right))),
                _ => rest,
            }
        }
        Some(Token::Minus) => {
            let rest = unary(&tokens, i);
            match rest {
                Ok(right) => Ok(Expr::Neg(Box::new(right))),
                _ => rest,
            }
        }
        _ => primary(&tokens, i),
    }
}

fn primary(tokens: &Vec<Token>, i: usize) -> Result<Expr, ParseError> {
    let curr_token = tokens.get(i);
    match curr_token {
        Some(Token::True) => Ok(Expr::ConstBool(true)),
        Some(Token::False) => Ok(Expr::ConstBool(false)),
        Some(Token::Number(num)) => Ok(Expr::ConstInt(num.clone())),
        _ => Err(ParseError::Error(get_curr_line(), "bruh".to_string())),
    }
}

//             Token::Let => {scan till end of decloration}
//             Token::If => {scan till Else ends?}
//             Token::Print
//             Token::Println
