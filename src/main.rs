use std::env;
use std::fs;
use std::io::{self, Error, ErrorKind, Write};

// mod evaler;
mod expr;
mod lexer;
mod parser;
mod stmt;
mod token;

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
    // lexer should return a result of type Result<Vec<token>, Error> instead of panic!
    let tokens = lexer::scan_tokens(contents);
    println!("{:?}", tokens);
    let ast = parser::parse(tokens);
    match ast {
        Ok(x) => println!("{:?}", x),
        Err(e) => println!("Error parsing: {:?}", e),
    }
}
