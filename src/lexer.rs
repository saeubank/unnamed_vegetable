use crate::token::Token;
use std::collections::HashMap;

pub fn scan_tokens(contents: &String) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    match scan_stored(&mut tokens, String::new(), contents) {
        Ok(()) => Ok(tokens),
        Err(e) => Err(e),
    }
}

#[derive(Debug)]
pub enum LexError {
    Error,
}

fn scan_stored(
    acc: &mut Vec<Token>,
    stored: String,
    to_be_scanned: &String,
) -> Result<(), LexError> {
    match to_be_scanned.chars().next() {
        Some(scan) => {
            match scan {
                '\n' | '\t' | '(' | ')' | '-' | '+' | '/' | '*' | ' ' => {
                    // make new fn called add_token(vec, str) -> result
                    add_token(acc, stored).and_then(|_| {
                        add_token(acc, scan.to_string())
                            .and_then(|_| scan_stored(acc, String::new(), &slice(to_be_scanned, 1)))
                    })
                }
                '\r' => scan_stored(acc, stored, &slice(to_be_scanned, 1)),
                '!' | '>' | '<' => {
                    add_token(acc, stored).and_then(|_| {
                        let mut to_be_scanned_copy = to_be_scanned.chars();
                        to_be_scanned_copy.next();
                        if to_be_scanned_copy.next() != Some('=') {
                            add_token(acc, scan.to_string()).and_then(|_| {
                                scan_stored(acc, String::new(), &slice(to_be_scanned, 1))
                            })
                        } else {
                            scan_stored(acc, scan.to_string(), &slice(to_be_scanned, 1))
                        }
                    })
                }
                '=' => {
                    match stored.as_str() {
                        "!" | ">" | "<" | "=" => {
                            add_token(acc, format!("{}{}", stored, scan)).and_then(|_| {
                                scan_stored(acc, String::new(), &slice(to_be_scanned, 1))
                            })
                        }
                        // turn stored into token
                        // if next is not "=" turn "=" into token,
                        // else stored becomes "="
                        _ => {
                            add_token(acc, stored).and_then(|_| {
                                let mut to_be_scanned_copy = to_be_scanned.chars();
                                to_be_scanned_copy.next();
                                if to_be_scanned_copy.next() != Some('=') {
                                    add_token(acc, String::from("=")).and_then(|_| {
                                        scan_stored(acc, String::new(), &slice(to_be_scanned, 1))
                                    })
                                } else {
                                    scan_stored(acc, String::from("="), &slice(to_be_scanned, 1))
                                }
                            })
                        }
                    }
                }
                _ => scan_stored(acc, format!("{}{}", stored, scan), &slice(to_be_scanned, 1)),
            }
        }
        _ => {
            add_token(acc, stored)
        }
    }
}

fn add_token(acc: &mut Vec<Token>, str_token: String) -> Result<(), LexError> {
    if !str_token.is_empty() && str_token != " " {
        // println!("{:?}", str_token.chars());
        to_token(str_token).map(|x| {
            acc.push(x);
        })
    } else {
        Ok(())
    }
}

fn slice(s: &String, pos: usize) -> String {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => s[pos..].to_string(),
        None => String::new(),
    }
}

fn to_token(str_token: String) -> Result<Token, LexError> {
    // remove null char
    let str_token = str_token.trim_matches(char::from(0)).to_string();
    // TODO: make this const or static
    let token_map: HashMap<String, Token> = [
        // ("\t", Token::Indent),
        ("\n", Token::NewLine),
        ("(", Token::LeftParen),
        (")", Token::RightParen),
        ("-", Token::Minus),
        ("+", Token::Plus),
        ("/", Token::Slash),
        ("*", Token::Star),
        ("!", Token::Bang),
        ("!=", Token::BangEqual),
        ("=", Token::Equal),
        ("==", Token::EqualEqual),
        (">", Token::Greater),
        (">=", Token::GreaterEqual),
        ("<", Token::Less),
        ("<=", Token::LessEqual),
        ("let", Token::Let),
        ("if", Token::If),
        ("else", Token::Else),
        ("true", Token::True),
        ("false", Token::False),
        ("print", Token::Print),
        ("println", Token::Println),
    ]
    .iter()
    .map(|(a, b)| (a.to_string(), b.clone()))
    .collect();

    match token_map.get(&str_token) {
        Some(token) => Ok(token.clone()),
        _ => {
            if is_digit(&str_token) {
                match str_token.parse::<i32>() {
                    Ok(num) => Ok(Token::Number(num)),
                    Err(e) => panic!("Error parsing i32: {}", e),
                }
            } else if is_alpha(&str_token) {
                Ok(Token::Identifier(str_token.clone()))
            } else {
                // show error on which line and why
                // panic!("Error lexing {:?}", str_token.chars())
                Err(LexError::Error)
            }
        }
    }
}

fn is_digit(s: &String) -> bool {
    s.chars().all(|x| "0123456789".contains(x))
}

fn is_alpha(s: &String) -> bool {
    s.chars()
        .all(|x| ('a'..='z').contains(&x) || ('A'..='Z').contains(&x) || x == '_')
}
