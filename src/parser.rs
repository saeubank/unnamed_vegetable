use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::token::Token;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, ParseError> {
    // turn tabs into {}?
    let mut ast = Vec::new();
    let mut parser = Parser::new(tokens);

    while !parser.at_end() {
        match statement(&mut parser) {
            Ok(stmt) => ast.push(stmt),
            Err(e) => return Err(e),
        }
    }
    Ok(ast)
}

fn expression(tokens: &mut Parser) -> Result<Expr, ParseError> {
    equality(tokens)
}

fn statement(tokens: &mut Parser) -> Result<Stmt, ParseError> {
    match tokens.curr() {
        Some(Token::Print) => {
            // should be the format "print" "(" expr ")" "newline"
            tokens.next();
            match expression(tokens) {
                Ok(expr) => Ok(Stmt::Print(expr)),
                Err(e) => Err(e),
            }
        }
        Some(Token::Println) => {
            tokens.next();
            match expression(tokens) {
                Ok(expr) => Ok(Stmt::Println(expr)),
                Err(e) => Err(e),
            }
        }
        Some(Token::Let) => {
            // till end of decloration
            // ident
            // if it is func then optional (ident)*
            // = sign
            tokens.next();
            match tokens.next() {
                Some(Token::Identifier(ident)) => {
                    let ident = ident.clone();
                    match tokens.next() {
                        Some(Token::Equal) => match expression(tokens) {
                            Ok(expr) => Ok(Stmt::ValDef(ident, expr)),
                            Err(e) => Err(e),
                        },
                        _ => Err(ParseError::Error(
                            tokens.get_line(),
                            "Expect \"=\" after identifier in let".to_string(),
                        )),
                    }
                }
                Some(t) => {
                    let tmp = t.clone();
                    Err(ParseError::Error(
                        tokens.get_line(),
                        format!("Expect identifier after let, got {:?}", tmp),
                    ))
                }
                None => Err(ParseError::Error(
                    tokens.get_line(),
                    "Expect identifier after let, got None".to_string(),
                )),
            }

            // match (ident)*

            // match expression(tokens) {
            //     Ok(expr) => Ok(Stmt::Println(expr)),
            //     Err(e) => Err(e)
            // }
        }
        _ => Err(ParseError::Error(
            tokens.get_line(),
            "Statement error".to_string(),
        )),
    }
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

    fn at_end(&self) -> bool {
        // println!("ind: {} len: {}", self.index, self.tokens.len());
        // needs to count tokens that are not newline?
        self.index >= self.tokens.iter().filter(|x| x != &&Token::NewLine).count()
    }
    fn curr(&mut self) -> Option<&Token> {
        // println!("curr: {:?}", self);
        self.tokens.get(self.index)
    }

    fn next(&mut self) -> Option<&Token> {
        let mut token = self.tokens.get(self.index);
        self.index += 1;
        // loop {
        //     if let Some(t) = token {
        //         if t == &Token::NewLine {
        //             token = self.tokens.get(self.index);
        //             self.index += 1;
        //         } else {
        //             break;
        //         }
        //     } else {
        //         break;
        //     }
        // }
        // println!("next: {:?}", self);
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
    match tokens.curr() {
        Some(Token::EqualEqual) => {
            tokens.next();
            match comparison(tokens) {
                Ok(right) => equality_helper(Expr::Equal(Box::new(expr), Box::new(right)), tokens),
                e @ Err(_) => e,
            }
        }
        Some(Token::BangEqual) => {
            tokens.next();
            match comparison(tokens) {
                Ok(right) => {
                    equality_helper(Expr::NotEqual(Box::new(expr), Box::new(right)), tokens)
                }
                e @ Err(_) => e,
            }
        }
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
    match tokens.curr() {
        Some(Token::Greater) => {
            tokens.next();
            match addition(tokens) {
                Ok(right) => {
                    comparison_helper(Expr::Greater(Box::new(expr), Box::new(right)), tokens)
                }
                e @ Err(_) => e,
            }
        }
        Some(Token::GreaterEqual) => {
            tokens.next();
            match addition(tokens) {
                Ok(right) => {
                    comparison_helper(Expr::GreaterEqual(Box::new(expr), Box::new(right)), tokens)
                }
                e @ Err(_) => e,
            }
        }
        Some(Token::Less) => {
            tokens.next();
            match addition(tokens) {
                Ok(right) => comparison_helper(Expr::Less(Box::new(expr), Box::new(right)), tokens),
                e @ Err(_) => e,
            }
        }
        Some(Token::LessEqual) => {
            tokens.next();
            match addition(tokens) {
                Ok(right) => {
                    comparison_helper(Expr::LessEqual(Box::new(expr), Box::new(right)), tokens)
                }
                e @ Err(_) => e,
            }
        }
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
    match tokens.curr() {
        Some(Token::Minus) => {
            tokens.next();
            match multiplication(tokens) {
                Ok(right) => addition_helper(Expr::Minus(Box::new(expr), Box::new(right)), tokens),
                e @ Err(_) => e,
            }
        }
        Some(Token::Plus) => {
            tokens.next();
            match multiplication(tokens) {
                Ok(right) => addition_helper(Expr::Plus(Box::new(expr), Box::new(right)), tokens),
                e @ Err(_) => e,
            }
        }
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
    match tokens.curr() {
        Some(Token::Slash) => {
            tokens.next();
            match unary(tokens) {
                Ok(right) => {
                    multiplication_helper(Expr::Div(Box::new(expr), Box::new(right)), tokens)
                }
                e @ Err(_) => e,
            }
        }
        Some(Token::Star) => {
            tokens.next();
            match unary(tokens) {
                Ok(right) => {
                    multiplication_helper(Expr::Mult(Box::new(expr), Box::new(right)), tokens)
                }
                e @ Err(_) => e,
            }
        }
        _ => Ok(expr),
    }
}

// match: ((! | -)*unary)
fn unary(tokens: &mut Parser) -> Result<Expr, ParseError> {
    match tokens.curr() {
        Some(Token::Bang) => {
            tokens.next();
            match unary(tokens) {
                Ok(right) => Ok(Expr::Not(Box::new(right))),
                e @ Err(_) => e,
            }
        }
        Some(Token::Minus) => {
            tokens.next();
            match unary(tokens) {
                Ok(right) => Ok(Expr::Neg(Box::new(right))),
                e @ Err(_) => e,
            }
        }
        _ => primary(tokens),
    }
}

fn primary(tokens: &mut Parser) -> Result<Expr, ParseError> {
    match tokens.next() {
        Some(Token::True) => Ok(Expr::ConstBool(true)),
        Some(Token::False) => Ok(Expr::ConstBool(false)),
        Some(Token::Number(num)) => Ok(Expr::ConstInt(*num)),
        Some(Token::Identifier(ident)) => Ok(Expr::Ident(ident.clone())),
        Some(Token::LeftParen) => {
            let expr = expression(tokens);
            match tokens.next() {
                Some(Token::RightParen) => match expr {
                    Ok(right) => Ok(Expr::Grouping(Box::new(right))),
                    e @ Err(_) => e,
                },
                Some(t) => {
                    let tmp = t.clone();
                    Err(ParseError::Error(
                        tokens.get_line(),
                        format!("Expected RightParen got: {:?}", tmp),
                    ))
                }
                None => Err(ParseError::Error(
                    tokens.get_line(),
                    "Got none in parsing expected RightParen".to_string(),
                )),
            }
        }
        Some(Token::If) => {
            let if_expr = expression(tokens);
            let true_expr = expression(tokens);
            match tokens.next() {
                Some(Token::Else) => match (if_expr, true_expr) {
                    (Ok(first), Ok(second)) => {
                        let false_expr = expression(tokens);
                        match false_expr {
                            Ok(third) => Ok(Expr::IfElse(
                                Box::new(first),
                                Box::new(second),
                                Box::new(third),
                            )),
                            e @ Err(_) => e,
                        }
                    }
                    (e @ Err(_), _) => e,
                    (_, e @ Err(_)) => e,
                },
                Some(t) => {
                    let tmp = t.clone();
                    Err(ParseError::Error(
                        tokens.get_line(),
                        format!("Expected else got: {:?}", tmp),
                    ))
                }
                None => Err(ParseError::Error(
                    tokens.get_line(),
                    "Got none in parsing expected RightParen".to_string(),
                )),
            }
        }
        Some(t) => {
            let tmp = t.clone();
            Err(ParseError::Error(
                tokens.get_line(),
                format!("Error parsing got: {:?}", tmp),
            ))
        }
        None => Err(ParseError::Error(
            tokens.get_line(),
            "Got none in parsing".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn negative_num() {
        let tokens = vec![Token::Minus, Token::Number(2)];
        let mut parser = Parser::new(tokens);
        let result = expression(&mut parser).unwrap();
        let correct = Expr::Neg(Box::new(Expr::ConstInt(2)));
        assert_eq!(correct, result);
    }

    #[test]
    fn num_plus_num() {
        let tokens = vec![Token::Number(2), Token::Plus, Token::Number(2)];
        let mut parser = Parser::new(tokens);
        let result = expression(&mut parser).unwrap();
        let correct = Expr::Plus(Box::new(Expr::ConstInt(2)), Box::new(Expr::ConstInt(2)));
        assert_eq!(correct, result);
    }

    #[test]
    fn num_minus_num() {
        let tokens = vec![Token::Number(2), Token::Minus, Token::Number(2)];
        let mut parser = Parser::new(tokens);
        let result = expression(&mut parser).unwrap();
        let correct = Expr::Minus(Box::new(Expr::ConstInt(2)), Box::new(Expr::ConstInt(2)));
        assert_eq!(correct, result);
    }
}
