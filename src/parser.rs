use crate::expr::Expr;
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
    let mut parser = Parser::new(tokens);
    equality(&mut parser)
}

#[derive(Debug)]
struct Parser {
    index: usize,
    tokens: Vec<Token>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            index: 0,
            tokens: tokens,
        }
    }

    fn curr(&mut self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn next(&mut self) -> Option<&Token> {
        let mut token = self.tokens.get(self.index);
        self.index += 1;
        loop {
            if let Some(t) = token {
                if t == &Token::NewLine {
                    token = self.tokens.get(self.index);
                    self.index += 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        println!("{:?}", self);
        token
    }

    fn get_line(&self) -> usize {
        let mut count = 1;
        for i in 0..self.index {
            match self.tokens.get(i) {
                Some(Token::NewLine) => count += 1,
                _ => {}
            }
        }
        count
    }
}

#[derive(Debug)]
pub enum ParseError {
    Error(usize, String),
}

// need to figure out how to handle when tokens.length() == 0 (assuming this needs to be handled)
// match: comp ((== | !=) comp)*
fn equality(tokens: &mut Parser) -> Result<Expr, ParseError> {
    match comparison(tokens) {
        Ok(expr) => equality_helper(expr, tokens),
        e @ Err(_) => e,
    }
}

fn equality_helper(expr: Expr, tokens: &mut Parser) -> Result<Expr, ParseError> {
    println!("In eq");
    match tokens.next() {
        Some(Token::EqualEqual) => match comparison(tokens) {
            Ok(right) => {
                equality_helper(Expr::Equal(Box::new(expr), Box::new(right)), tokens)
            }
            e @ Err(_) => e,
        },
        Some(Token::BangEqual) => match comparison(tokens) {
            Ok(right) => equality_helper(Expr::NotEqual(Box::new(expr), Box::new(right)), tokens),
            e @ Err(_) => e,
        },
        _ => Ok(expr),
    }
}

// match: add ((> | >= | < | <=) add)*
fn comparison(tokens: &mut Parser) -> Result<Expr, ParseError> {
    match addition(tokens) {
        Ok(expr) => comparison_helper(expr, tokens),
        e @ Err(_) => e,
    }
}

fn comparison_helper(expr: Expr, tokens: &mut Parser) -> Result<Expr, ParseError> {
    println!("In cmp");
    match tokens.next() {
        Some(Token::Greater) => match addition(tokens) {
            Ok(right) => {
                comparison_helper(Expr::Greater(Box::new(expr), Box::new(right)), tokens)
            }
            e @ Err(_) => e,
        },
        Some(Token::GreaterEqual) => match addition(tokens) {
            Ok(right) => {
                comparison_helper(Expr::GreaterEqual(Box::new(expr), Box::new(right)), tokens)
            }
            e @ Err(_) => e,
        },
        Some(Token::Less) => match addition(tokens) {
            Ok(right) => comparison_helper(Expr::Less(Box::new(expr), Box::new(right)), tokens),
            e @ Err(_) => e,
        },
        Some(Token::LessEqual) => match addition(tokens) {
            Ok(right) => {
                comparison_helper(Expr::LessEqual(Box::new(expr), Box::new(right)), tokens)
            }
            e @ Err(_) => e,
        },
        _ => Ok(expr),
    }
}

// match: mult ((- | +) mult)*
fn addition(tokens: &mut Parser) -> Result<Expr, ParseError> {
    match multiplication(tokens) {
        Ok(expr) => addition_helper(expr, tokens),
        e @ Err(_) => e,
    }
}

fn addition_helper(expr: Expr, tokens: &mut Parser) -> Result<Expr, ParseError> {
    println!("In add");
    match tokens.next() {
        Some(Token::Minus) => match multiplication(tokens) {
            Ok(right) => {
                addition_helper(Expr::Minus(Box::new(expr), Box::new(right)), tokens)
            }
            e @ Err(_) => e,
        },
        Some(Token::Plus) => match multiplication(tokens) {
            Ok(right) => addition_helper(Expr::Plus(Box::new(expr), Box::new(right)), tokens),
            e @ Err(_) => e,
        },
        _ => Ok(expr),
    }
}

// match: unary ((/ | *) unary)*
fn multiplication(tokens: &mut Parser) -> Result<Expr, ParseError> {
    match unary(tokens) {
        Ok(expr) => multiplication_helper(expr, tokens),
        e @ Err(_) => e,
    }
}

fn multiplication_helper(expr: Expr, tokens: &mut Parser) -> Result<Expr, ParseError> {
    println!("In mult");
    match tokens.next() {
        Some(Token::Slash) => match unary(tokens) {
            Ok(right) => {
                multiplication_helper(Expr::Div(Box::new(expr), Box::new(right)), tokens)
            }
            e @ Err(_) => e,
        },
        Some(Token::Star) => match unary(tokens) {
            Ok(right) => multiplication_helper(Expr::Mult(Box::new(expr), Box::new(right)), tokens),
            e @ Err(_) => e,
        },
        _ => Ok(expr),
    }
}

// match: ((! | -)*unary)
fn unary(tokens: &mut Parser) -> Result<Expr, ParseError> {
    println!("In unary");
    let curr_token = tokens.curr();
    match curr_token {
        Some(Token::Bang) => match unary(tokens) {
            Ok(right) => Ok(Expr::Not(Box::new(right))),
            e @ Err(_) => e,
        },
        Some(Token::Minus) => match unary(tokens) {
            Ok(right) => Ok(Expr::Neg(Box::new(right))),
            e @ Err(_) => e,
        },
        _ => primary(tokens),
    }
}

fn primary(tokens: &mut Parser) -> Result<Expr, ParseError> {
    println!("In primary {:?}", tokens);
    let curr_token = tokens.curr();
    match curr_token {
        Some(Token::True) => Ok(Expr::ConstBool(true)),
        Some(Token::False) => Ok(Expr::ConstBool(false)),
        Some(Token::Number(num)) => Ok(Expr::ConstInt(num.clone())),
        Some(t) => Err(ParseError::Error(
            // tokens.get_line(),
            0,
            format!("{:?}", t),
        )),
        None => Err(ParseError::Error(
            tokens.get_line(),
            "Got none in parsing".to_string(),
        )),
    }
}
