use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Error, ErrorKind, Write};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        Err(Error::new(ErrorKind::Other, "Useage: unv [script]"))
    } else if args.len() == 2 {
        run_file(&args[1])
    } else {
        run_prompt()
    }
}

// For running files
fn run_file(file_name: &String) -> Result<(), Error> {
    let contents = fs::read_to_string(file_name)?;
    run(&contents);
    Ok(())
}

// For REPL
fn run_prompt() -> Result<(), Error> {
    let mut buffer = String::new();
    while buffer.trim() != "exit()" {
        run(&buffer);
        buffer.clear();

        print!("unv> ");

        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;
    }
    Ok(())
}

fn run(contents: &String) {
    let tokens = scan_tokens(contents);
    for token in tokens {
        println!("{:?}", token);
    }
    println!("{}", contents);
}

#[derive(Debug, Clone)]
enum Token {
    // White space
    Indent,
    NewLine,

    // Single-character tokens.
    LeftParen,
    RightParen,
    Minus,
    Plus,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // AndAnd
    // PipePipe

    // Literals.
    Identifier(String),
    // String,
    Number(i32),

    // Keywords.
    Let,
    If,
    Else,
    True,
    False,
    Print,
    Println,
    // STRUCT, RETURN, YEILD, monads
}

fn scan_tokens(contents: &String) -> Vec<Token> {
    scan_stored(Vec::new(), String::new(), contents)
}

fn scan_stored(acc: Vec<Token>, stored: String, to_be_scanned: &String) -> Vec<Token> {
    match to_be_scanned.chars().next() {
        Some(scan) => {
            match scan {
                '\t' | '\n' | '(' | ')' | '-' | '+' | '/' | '*' | ' ' => { // need special case for ' '?
                    scan_stored_helper(acc, stored, to_be_scanned, scan) 
                }
                '\r' => scan_stored(acc, stored, &crop_letters(to_be_scanned, 1)),
                // //2 or more
                // "!"
                // "!="
                // "="
                // "=="
                // ">"
                // ">="
                // "<"
                // "<="
                _ => scan_stored(acc, format!("{}{}", stored, scan), &crop_letters(to_be_scanned, 1)),
            }
        }
        _ => acc,
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
    scan_stored(
        [acc, tmp].concat(),
        String::new(),
        &crop_letters(to_be_scanned, 1),
    )
}

fn crop_letters(s: &String, pos: usize) -> String {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => s[pos..].to_string(),
        None => String::new(),
    }
}

fn to_token(str_token: String) -> Option<Token> {
    let token_map: HashMap<_, _> = vec![
        ("\t", Token::Indent),
        ("\n", Token::NewLine),
        ("(", Token::RightParen),
        (")", Token::LeftParen),
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
    .into_iter()
    .map(|(a, b)| (String::from(a), b))
    .collect();

    if str_token.is_empty() || str_token == " " {
        return None
    }

    match token_map.get(&str_token) {
        Some(token) => Some(token.clone()),
        _ => {
            if str_token.chars().all(|x| "0123456789".contains(x)) {
                match str_token.parse::<i32>() {
                    Ok(num) => Some(Token::Number(num)),
                    Err(e) => panic!("Error parsing i32: {}", e),
                }
            } else {
                Some(Token::Identifier(str_token.clone()))
            }
        }
    }
}
