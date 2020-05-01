use crate::token::Token;
use std::collections::HashMap;

pub fn scan_tokens(contents: &String) -> Vec<Token> {
    scan_stored(Vec::new(), String::new(), contents)
}

fn scan_stored(acc: Vec<Token>, stored: String, to_be_scanned: &String) -> Vec<Token> {
    match to_be_scanned.chars().next() {
        Some(scan) => {
            match scan {
                // Does ' ' need a speical case?
                '\n' | '\t' | '(' | ')' | '-' | '+' | '/' | '*' | ' ' => {
                    scan_stored_helper(acc, stored, to_be_scanned, scan)
                }
                '\r' => scan_stored(acc, stored, &slice(to_be_scanned, 1)),
                '!' | '>' | '<' => {
                    let mut tmp = Vec::new();
                    if let Some(x) = to_token(stored) {
                        tmp.push(x);
                    }
                    let mut to_be_scanned_copy = to_be_scanned.chars();
                    to_be_scanned_copy.next();
                    if to_be_scanned_copy.next() != Some('=') {
                        if let Some(x) = to_token(scan.to_string()) {
                            tmp.push(x);
                        }

                        scan_stored([acc, tmp].concat(), String::new(), &slice(to_be_scanned, 1))
                    } else {
                        scan_stored(
                            [acc, tmp].concat(),
                            scan.to_string(),
                            &slice(to_be_scanned, 1),
                        )
                    }
                }
                '=' => {
                    match stored.as_str() {
                        // turn stored+scan into token
                        "!" | ">" | "<" | "=" => scan_stored_helper(
                            acc,
                            format!("{}{}", stored, scan),
                            to_be_scanned,
                            '\0',
                        ),
                        // turn stored into token
                        // if next is not "=" turn "=" into token,
                        // else stored becomes "="
                        _ => {
                            let mut tmp = Vec::new();
                            if let Some(x) = to_token(stored) {
                                tmp.push(x);
                            }
                            let mut to_be_scanned_copy = to_be_scanned.chars();
                            to_be_scanned_copy.next();
                            if to_be_scanned_copy.next() != Some('=') {
                                if let Some(x) = to_token(String::from("=")) {
                                    tmp.push(x);
                                }
                                scan_stored(
                                    [acc, tmp].concat(),
                                    String::new(),
                                    &slice(to_be_scanned, 1),
                                )
                            } else {
                                scan_stored(
                                    [acc, tmp].concat(),
                                    String::from("="),
                                    &slice(to_be_scanned, 1),
                                )
                            }
                        }
                    }
                }
                _ => scan_stored(acc, format!("{}{}", stored, scan), &slice(to_be_scanned, 1)),
            }
        }
        _ => {
            let mut tmp = Vec::new();
            if let Some(x) = to_token(stored) {
                tmp.push(x);
            }
            [acc, tmp].concat()
        }
    }
}

fn scan_stored_helper(
    acc: Vec<Token>,
    stored: String,
    to_be_scanned: &String,
    cur_char: char,
) -> Vec<Token> {
    let mut tmp = Vec::new();
    if let Some(x) = to_token(stored) {
        tmp.push(x);
    }
    if let Some(x) = to_token(cur_char.to_string()) {
        tmp.push(x);
    }
    scan_stored([acc, tmp].concat(), String::new(), &slice(to_be_scanned, 1))
}

fn slice(s: &String, pos: usize) -> String {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => s[pos..].to_string(),
        None => String::new(),
    }
}

fn to_token(str_token: String) -> Option<Token> {
    // make this const and move out of fn or make static
    let str_token = str_token.trim_matches(char::from(0)).to_string();
    let token_map: HashMap<String, Token> = [
        // ("\t", Token::Indent),
        // ("\n", Token::NewLine),
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

    if str_token == "\n" {
        Some(Token::NewLine)
    } else if str_token.trim().is_empty() {
        None
    } else {
        match token_map.get(&str_token) {
            Some(token) => Some(token.clone()),
            _ => {
                if is_digit(&str_token) {
                    match str_token.parse::<i32>() {
                        Ok(num) => Some(Token::Number(num)),
                        Err(e) => panic!("Error parsing i32: {}", e),
                    }
                } else if is_alpha(&str_token) {
                    Some(Token::Identifier(str_token.clone()))
                } else {
                    // show error on which line and why
                    panic!("Error lexing {:?}", str_token.chars())
                }
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
