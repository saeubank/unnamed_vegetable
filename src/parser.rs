use crate::token::Token;

// pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParseError> {
//     // turn newline+ into newline?
//     // turn tabs into {}?
//     // turn fn ... -> Expr to fn ... -> Result<Expr, error>
//     // let parser = Parser::new()
//     let mut parser = Parser::new(&tokens);
//     parser.parse()
// }

// #[derive(Debug)]
// pub enum Expr {
//     // Ident(String),
//     ConstInt(i32),
//     ConstBool(bool),
//     Equal(Box<Expr>, Box<Expr>),
//     NotEqual(Box<Expr>, Box<Expr>),
//     Greater(Box<Expr>, Box<Expr>),
//     GreaterEqual(Box<Expr>, Box<Expr>),
//     Less(Box<Expr>, Box<Expr>),
//     LessEqual(Box<Expr>, Box<Expr>),
//     Minus(Box<Expr>, Box<Expr>),
//     Plus(Box<Expr>, Box<Expr>),
//     Div(Box<Expr>, Box<Expr>),
//     Mult(Box<Expr>, Box<Expr>),
//     Not(Box<Expr>),
//     Neg(Box<Expr>),
// }

// #[derive(Debug)]
// pub enum ParseError {
//     Error(usize, String),
// }

// struct Parser<'a> {
//     i: usize,
//     tokens: &'a Vec<Token>
// }

// impl <'a> Parser<'a> {
//     pub fn new(tokens: &'a Vec<Token>) -> Self {
//         Self {
//             i: 0,
//             tokens: tokens
//         }
//     }

//     pub fn parse(&mut self) -> Result<Expr, ParseError> {
//         self.equality()
//     }

//     fn equality(&mut self) -> Result<Expr, ParseError> {
//         let mut expr = match self.primary() {
//             Ok(x) => x,
//             Err(e) => return Err(e),
//         };
//         loop {
//             // println!("in equality loop {}", i);
//             let curr_token = self.tokens.get(self.i);
//             self.i += 1;
//             match curr_token {
//                 Some(Token::EqualEqual) => {
//                     let rest = self.primary();
//                     match rest {
//                         Ok(right) => expr = Expr::Equal(Box::new(expr), Box::new(right)),
//                         _ => return rest,
//                     }
//                 }
//                 Some(Token::BangEqual) => {
//                     let rest = self.primary();
//                     match rest {
//                         Ok(right) => expr = Expr::NotEqual(Box::new(expr), Box::new(right)),
//                         _ => return rest,
//                     }
//                 }
//                 _ => break,
//             }
//         }
//         Ok(expr)
//     }

// fn comparison(&mut self) -> Result<Expr, ParseError> {
//     let mut expr = match self.addition() {
//         Ok(x) => x,
//         Err(e) => return Err(e),
//     };
//     loop {
//         // println!("in comparison loop {}", i);
//         let curr_token = self.tokens.get(self.i);
//         self.i += 1;
//         match curr_token {
//             Some(Token::Greater) => {
//                 let rest = self.addition();
//                 match rest {
//                     Ok(right) => expr = Expr::Greater(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             Some(Token::GreaterEqual) => {
//                 let rest = self.addition();
//                 match rest {
//                     Ok(right) => expr = Expr::GreaterEqual(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             Some(Token::Less) => {
//                 let rest = self.addition();
//                 match rest {
//                     Ok(right) => expr = Expr::Less(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             Some(Token::LessEqual) => {
//                 let rest = self.addition();
//                 match rest {
//                     Ok(right) => expr = Expr::LessEqual(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             _ => break,
//         }
//     }
//     Ok(expr)
// }

// fn addition(&mut self) -> Result<Expr, ParseError> {
//     let mut expr = match self.multiplication() {
//         Ok(x) => x,
//         Err(e) => return Err(e),
//     };
//     loop {
//         // println!("in addition loop {}", i);
//         let curr_token = self.tokens.get(self.i);
//         self.i += 1;
//         match curr_token {
//             Some(Token::Minus) => {
//                 let rest = self.multiplication();
//                 match rest {
//                     Ok(right) => expr = Expr::Minus(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             Some(Token::Plus) => {
//                 let rest = self.multiplication();
//                 match rest {
//                     Ok(right) => expr = Expr::Plus(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             _ => break,
//         }
//     }
//     Ok(expr)
// }

// fn multiplication(&mut self) -> Result<Expr, ParseError> {
//     let mut expr = match self.unary() {
//         Ok(x) => x,
//         Err(e) => return Err(e),
//     };
//     loop {
//         // println!("in multiplication loop {}", i);
//         let curr_token = self.tokens.get(self.i);
//         self.i += 1;
//         match curr_token {
//             Some(Token::Slash) => {
//                 let rest = self.unary();
//                 match rest {
//                     Ok(right) => expr = Expr::Div(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             Some(Token::Star) => {
//                 let rest = self.unary();
//                 match rest {
//                     Ok(right) => expr = Expr::Mult(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             _ => break,
//         }
//     }
//     Ok(expr)
// }

// fn unary(&mut self) -> Result<Expr, ParseError> {
//     let curr_token = self.tokens.get(self.i);
//     // println!("in unary {}", i);
//     match curr_token {
//         Some(Token::Bang) => {
//             self.i += 1;
//             let rest = self.unary();
//             match rest {
//                 Ok(right) => Ok(Expr::Not(Box::new(right))),
//                 _ => rest,
//             }
//         }
//         Some(Token::Minus) => {
//             self.i += 1;
//             let rest = self.unary();
//             match rest {
//                 Ok(right) => Ok(Expr::Neg(Box::new(right))),
//                 _ => rest,
//             }
//         }
//         _ => self.primary(),
//     }
// }

//     fn primary(&mut self) -> Result<Expr, ParseError> {
//         let curr_token = self.tokens.get(self.i);
//         // println!("in primary {}", i);
//         match curr_token {
//             Some(Token::True) => Ok(Expr::ConstBool(true)),
//             Some(Token::False) => Ok(Expr::ConstBool(false)),
//             Some(Token::Number(num)) => Ok(Expr::ConstInt(num.clone())),
//             Some(t) => Err(ParseError::Error(
//                 self.get_curr_line(),
//                 format!("{:?}", t),
//             )),
//             _ => Err(ParseError::Error(
//                 self.get_curr_line(),
//                 "Got none in parsing".to_string(),
//             )),
//         }
//     }

//     fn get_curr_line(&self) -> usize {
//         let mut count = 0;
//         for _ in 0..self.i {
//             let token = self.tokens.get(self.i);
//             match token {
//                 Some(Token::NewLine) => count += 1,
//                 _ => {}
//             }
//         }
//         count
//     }
// }

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

fn get_curr_line(tokens: &Vec<Token>, i: usize) -> usize {
    let mut count = 0;
    for i in 0..i {
        let token = tokens.get(i);
        match token {
            Some(Token::NewLine) => count += 1,
            _ => {}
        }
    }
    count
}

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

fn equality(tokens: &[Token]) -> Result<Expr, ParseError> {
    let expr = match primary(&tokens) {
        Ok(x) => x,
        e @ Err(_) => return e,
    };
    equality_helper(expr, &tokens[1..])
}

// match patters ((== | !=) expr)*
fn equality_helper(expr: Expr, tokens: &[Token]) -> Result<Expr, ParseError> {
    match tokens.get(0) {
        Some(Token::EqualEqual) => match primary(&tokens[1..]) {
            Ok(right) => {
                equality_helper(Expr::Equal(Box::new(expr), Box::new(right)), &tokens[2..])
            }
            e @ Err(_) => e,
        },
        Some(Token::BangEqual) => match primary(&tokens[1..]) {
            Ok(right) => {
                equality_helper(Expr::NotEqual(Box::new(expr), Box::new(right)), &tokens[2..])
            }
            e @ Err(_) => e,
        },

        _ => Ok(expr),
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
        _ => Err(ParseError::Error(
            // get_curr_line(&tokens, i),
            0,
            "Got none in parsing".to_string(),
        )),
    }
}

// fn equality(tokens: &Vec<Token>, i: usize) -> Result<Expr, ParseError> {
//     let mut expr = match primary(&tokens, i) {
//         Ok(x) => x,
//         Err(e) => return Err(e),
//     };
//     println!("test {:?}", expr);
//     loop {
//         // println!("in equality loop {}", i);
//         let mut i = i + 1;
//         let curr_token = tokens.get(i);

//         match curr_token {
//             Some(Token::EqualEqual) => {
//                 i += 1;
//                 let rest = primary(&tokens, i);
//                 match rest {
//                     Ok(right) => expr = Expr::Equal(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             Some(Token::BangEqual) => {
//                 i += 1
//                 let rest = primary(&tokens, i);
//                 match rest {
//                     Ok(right) => expr = Expr::NotEqual(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             _ => break,
//         }
//     }
//     Ok(expr)
// }

// fn comparison(tokens: &Vec<Token>, i: usize) -> Result<Expr, ParseError> {
//     let mut expr = match addition(&tokens, i) {
//         Ok(x) => x,
//         Err(e) => return Err(e),
//     };
//     loop {
//         // println!("in comparison loop {}", i);
//         let curr_token = tokens.get(i);
//         let i = i + 1;
//         match curr_token {
//             Some(Token::Greater) => {
//                 let rest = addition(&tokens, i);
//                 match rest {
//                     Ok(right) => expr = Expr::Greater(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             Some(Token::GreaterEqual) => {
//                 let rest = addition(&tokens, i);
//                 match rest {
//                     Ok(right) => expr = Expr::GreaterEqual(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             Some(Token::Less) => {
//                 let rest = addition(&tokens, i);
//                 match rest {
//                     Ok(right) => expr = Expr::Less(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             Some(Token::LessEqual) => {
//                 let rest = addition(&tokens, i);
//                 match rest {
//                     Ok(right) => expr = Expr::LessEqual(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             _ => break,
//         }
//     }
//     Ok(expr)
// }

// fn addition(tokens: &Vec<Token>, i: usize) -> Result<Expr, ParseError> {
//     let mut expr = match multiplication(&tokens, i) {
//         Ok(x) => x,
//         Err(e) => return Err(e),
//     };
//     loop {
//         // println!("in addition loop {}", i);
//         let curr_token = tokens.get(i);
//         let i = i + 1;
//         match curr_token {
//             Some(Token::Minus) => {
//                 let rest = multiplication(&tokens, i);
//                 match rest {
//                     Ok(right) => expr = Expr::Minus(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             Some(Token::Plus) => {
//                 let rest = multiplication(&tokens, i);
//                 match rest {
//                     Ok(right) => expr = Expr::Plus(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             _ => break,
//         }
//     }
//     Ok(expr)
// }

// fn multiplication(tokens: &Vec<Token>, i: usize) -> Result<Expr, ParseError> {
//     let mut expr = match unary(&tokens, i) {
//         Ok(x) => x,
//         Err(e) => return Err(e),
//     };
//     loop {
//         // println!("in multiplication loop {}", i);
//         let curr_token = tokens.get(i);
//         let i = i + 1;
//         match curr_token {
//             Some(Token::Slash) => {
//                 let rest = unary(&tokens, i);
//                 match rest {
//                     Ok(right) => expr = Expr::Div(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             Some(Token::Star) => {
//                 let rest = unary(&tokens, i);
//                 match rest {
//                     Ok(right) => expr = Expr::Mult(Box::new(expr), Box::new(right)),
//                     _ => return rest,
//                 }
//             }
//             _ => break,
//         }
//     }
//     Ok(expr)
// }

// fn unary(tokens: &Vec<Token>, i: usize) -> Result<Expr, ParseError> {
//     let curr_token = tokens.get(i);
//     // println!("in unary {}", i);
//     match curr_token {
//         Some(Token::Bang) => {
//             let rest = unary(&tokens, i + 1);
//             match rest {
//                 Ok(right) => Ok(Expr::Not(Box::new(right))),
//                 _ => rest,
//             }
//         }
//         Some(Token::Minus) => {
//             let rest = unary(&tokens, i + 1);
//             match rest {
//                 Ok(right) => Ok(Expr::Neg(Box::new(right))),
//                 _ => rest,
//             }
//         }
//         _ => primary(&tokens, i),
//     }
// }

// fn primary(tokens: &Vec<Token>, i: usize) -> Result<Expr, ParseError> {
//     let curr_token = tokens.get(i);
//     // println!("in primary {}", i);
//     // i++
//     match curr_token {
//         Some(Token::True) => Ok(Expr::ConstBool(true)),
//         Some(Token::False) => Ok(Expr::ConstBool(false)),
//         Some(Token::Number(num)) => Ok(Expr::ConstInt(num.clone())),
//         Some(t) => Err(ParseError::Error(
//             get_curr_line(&tokens, i),
//             format!("{:?}", t),
//         )),
//         _ => Err(ParseError::Error(
//             get_curr_line(&tokens, i),
//             "Got none in parsing".to_string(),
//         )),
//     }
// }
