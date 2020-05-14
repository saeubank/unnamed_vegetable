use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::token::Token;

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
        self.index + 1 >= self.tokens.len()
        // self.index >= self.tokens.iter().filter(|x| x != &&Token::NewLine).count()
    }

    fn curr(&mut self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.index);
        self.index += 1;
        token
    }

    fn next_ignore_newline(&mut self) -> Option<&Token> {
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
        // println!("next: {:?}", self);
        token
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.index + 1)
    }

    fn get_line(&self) -> usize {
        let mut count = 1;
        for i in 0..self.index {
            // println!("{:?}", self.tokens.get(i));
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

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, ParseError> {
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
            tokens.next();
            expression(tokens).map(|expr| Stmt::Print(expr))
        }
        Some(Token::Println) => {
            tokens.next();
            expression(tokens).map(|expr| Stmt::Println(expr))
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
                        Some(Token::LeftParen) => match tokens.next() {
                            Some(Token::Identifier(param)) => {
                                let param = param.clone();
                                match tokens.next() {
                                    Some(Token::RightParen) => fndef(ident, param, tokens),
                                    Some(t) => {
                                        let tmp = t.clone();
                                        Err(ParseError::Error(
                                            tokens.get_line(),
                                            format!("Expect \")\" after ident, got {:?}", tmp),
                                        ))
                                    }
                                    None => Err(ParseError::Error(
                                        tokens.get_line(),
                                        "Expect \")\" after ident, got None".to_string(),
                                    )),
                                }
                            }
                            Some(t) => {
                                let tmp = t.clone();
                                Err(ParseError::Error(
                                    tokens.get_line(),
                                    format!("Expect ident after \"(\", got {:?}", tmp),
                                ))
                            }
                            None => Err(ParseError::Error(
                                tokens.get_line(),
                                "Expect ident after \"(\", got None".to_string(),
                            )),
                        },
                        Some(Token::Equal) => match expression(tokens) {
                            Ok(expr) => Ok(Stmt::ValDef(ident, expr)),
                            Err(e) => Err(e),
                        },
                        Some(t) => {
                            let tmp = t.clone();
                            Err(ParseError::Error(
                                tokens.get_line(),
                                format!("Expect \"=\" after identifier in let, got {:?}", tmp),
                            ))
                        }
                        None => Err(ParseError::Error(
                            tokens.get_line(),
                            "Expect \"=\" after identifier in let, got None".to_string(),
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
        }
        Some(Token::NewLine) => {
            tokens.next();
            statement(tokens)
        }
        Some(t) => {
            let tmp = t.clone();
            Err(ParseError::Error(
                tokens.get_line(),
                format!("Expect let, print, or println, got {:?}", tmp),
            ))
        }
        None => Err(ParseError::Error(
            tokens.get_line(),
            "Statement error".to_string(),
        )),
    }
}

fn fndef(ident: String, param: String, tokens: &mut Parser) -> Result<Stmt, ParseError> {
    let fndef_rec = fndef_helper(Vec::new(), tokens);
    match fndef_rec {
        Ok(params) => match expression(tokens) {
            Ok(expr) => Ok(Stmt::FunDef(ident, [vec![param], params].concat(), expr)),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn fndef_helper(params: Vec<String>, tokens: &mut Parser) -> Result<Vec<String>, ParseError> {
    match tokens.next() {
        // recurse in this case
        Some(Token::LeftParen) => match tokens.next() {
            Some(Token::Identifier(param)) => {
                let param = param.clone();
                match tokens.next() {
                    Some(Token::RightParen) => fndef_helper([params, vec![param]].concat(), tokens),
                    Some(t) => {
                        let tmp = t.clone();
                        Err(ParseError::Error(
                            tokens.get_line(),
                            format!("Expect \")\" after ident, got {:?}", tmp),
                        ))
                    }
                    None => Err(ParseError::Error(
                        tokens.get_line(),
                        "Expect \")\" after ident, got None".to_string(),
                    )),
                }
            }
            Some(t) => {
                let tmp = t.clone();
                Err(ParseError::Error(
                    tokens.get_line(),
                    format!("Expect ident after \"(\", got {:?}", tmp),
                ))
            }
            None => Err(ParseError::Error(
                tokens.get_line(),
                "Expect ident after \"(\", got None".to_string(),
            )),
        },
        //return in this case
        Some(Token::Equal) => Ok(params),
        Some(t) => {
            let tmp = t.clone();
            Err(ParseError::Error(
                tokens.get_line(),
                format!("Expect \"=\" after identifier in let, got {:?}", tmp),
            ))
        }
        None => Err(ParseError::Error(
            tokens.get_line(),
            "Expect \"=\" after identifier in let, got None".to_string(),
        )),
    }
}

// match: comp ((== | !=) comp)*
fn equality(tokens: &mut Parser) -> Result<Expr, ParseError> {
    comparison(tokens).and_then(|expr| equality_helper(expr, tokens))
}

fn equality_helper(expr: Expr, tokens: &mut Parser) -> Result<Expr, ParseError> {
    match tokens.curr() {
        Some(Token::EqualEqual) => {
            tokens.next();
            comparison(tokens).and_then(|right| {
                equality_helper(Expr::Equal(Box::new(expr), Box::new(right)), tokens)
            })
        }
        Some(Token::BangEqual) => {
            tokens.next();
            comparison(tokens).and_then(|right| {
                equality_helper(Expr::NotEqual(Box::new(expr), Box::new(right)), tokens)
            })
        }
        _ => Ok(expr),
    }
}

// match: add ((> | >= | < | <=) add)*
fn comparison(tokens: &mut Parser) -> Result<Expr, ParseError> {
    addition(tokens).and_then(|expr| comparison_helper(expr, tokens))
}

fn comparison_helper(expr: Expr, tokens: &mut Parser) -> Result<Expr, ParseError> {
    match tokens.curr() {
        Some(Token::Greater) => {
            tokens.next();
            addition(tokens).and_then(|right| {
                comparison_helper(Expr::Greater(Box::new(expr), Box::new(right)), tokens)
            })
        }
        Some(Token::GreaterEqual) => {
            tokens.next();
            addition(tokens).and_then(|right| {
                comparison_helper(Expr::GreaterEqual(Box::new(expr), Box::new(right)), tokens)
            })
        }
        Some(Token::Less) => {
            tokens.next();
            addition(tokens).and_then(|right| {
                comparison_helper(Expr::Less(Box::new(expr), Box::new(right)), tokens)
            })
        }
        Some(Token::LessEqual) => {
            tokens.next();
            addition(tokens).and_then(|right| {
                comparison_helper(Expr::LessEqual(Box::new(expr), Box::new(right)), tokens)
            })
        }
        _ => Ok(expr),
    }
}

// match: mult ((- | +) mult)*
fn addition(tokens: &mut Parser) -> Result<Expr, ParseError> {
    multiplication(tokens).and_then(|expr| addition_helper(expr, tokens))
}

fn addition_helper(expr: Expr, tokens: &mut Parser) -> Result<Expr, ParseError> {
    match tokens.curr() {
        Some(Token::Minus) => {
            tokens.next();
            multiplication(tokens).and_then(|right| {
                addition_helper(Expr::Minus(Box::new(expr), Box::new(right)), tokens)
            })
        }
        Some(Token::Plus) => {
            tokens.next();
            multiplication(tokens).and_then(|right| {
                addition_helper(Expr::Plus(Box::new(expr), Box::new(right)), tokens)
            })
        }
        _ => Ok(expr),
    }
}

// match: unary ((/ | *) unary)*
fn multiplication(tokens: &mut Parser) -> Result<Expr, ParseError> {
    unary(tokens).and_then(|expr| multiplication_helper(expr, tokens))
}

fn multiplication_helper(expr: Expr, tokens: &mut Parser) -> Result<Expr, ParseError> {
    match tokens.curr() {
        Some(Token::Slash) => {
            tokens.next();
            unary(tokens).and_then(|right| {
                multiplication_helper(Expr::Div(Box::new(expr), Box::new(right)), tokens)
            })
        }
        Some(Token::Star) => {
            tokens.next();
            unary(tokens).and_then(|right| {
                multiplication_helper(Expr::Mult(Box::new(expr), Box::new(right)), tokens)
            })
        }
        _ => Ok(expr),
    }
}

// match: ((! | -)*unary)
fn unary(tokens: &mut Parser) -> Result<Expr, ParseError> {
    match tokens.curr() {
        Some(Token::Bang) => {
            tokens.next();
            unary(tokens).map(|right| Expr::Not(Box::new(right)))
        }
        Some(Token::Minus) => {
            tokens.next();
            unary(tokens).map(|right| Expr::Neg(Box::new(right)))
        }
        _ => primary(tokens),
    }
}

fn primary(tokens: &mut Parser) -> Result<Expr, ParseError> {
    match tokens.next() {
        Some(Token::NewLine) => primary(tokens),
        Some(Token::True) => Ok(Expr::ConstBool(true)),
        Some(Token::False) => Ok(Expr::ConstBool(false)),
        Some(Token::Number(num)) => Ok(Expr::ConstInt(*num)),
        Some(Token::Identifier(ident)) => {
            // Need to handle case when is funcall
            // this case is when u have ident (ident)+
            let ident = ident.clone();
            match tokens.curr() {
                Some(Token::LeftParen) => fn_call(Expr::Ident(ident), tokens),
                _ => Ok(Expr::Ident(ident)),
            }
        }
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
            match tokens.next_ignore_newline() {
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

fn fn_call(ident: Expr, tokens: &mut Parser) -> Result<Expr, ParseError> {
    match tokens.next() {
        Some(Token::LeftParen) => {
            let fn_expr = expression(tokens);
            match fn_expr {
                Ok(expr) => match tokens.next() {
                    Some(Token::RightParen) => {
                        let fc = Expr::FunCall(Box::new(ident), Box::new(expr));
                        match tokens.peek() {
                            Some(Token::LeftParen) => fn_call(fc, tokens),
                            _ => Ok(fc),
                        }
                    }
                    Some(t) => {
                        let tmp = t.clone();
                        Err(ParseError::Error(
                            tokens.get_line(),
                            format!("Expected \")\" got {:?}", tmp),
                        ))
                    }
                    None => Err(ParseError::Error(
                        tokens.get_line(),
                        "Expected \")\" got none".to_string(),
                    )),
                },
                e @ Err(_) => e,
            }
        }
        _ => panic!("This shouldnt be possible"),
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
