use std::io::{BufRead, Write};
use std::process::exit;
use std::{env, fs, io};

mod lexer;
use crate::lexer::*;
mod test;

fn run_file(file_path: &str) -> Result<(), io::Error> {
    match fs::read_to_string(file_path) {
        Ok(content) => return run(&content),
        Err(msg) => return Err(io::Error::new(io::ErrorKind::Other, msg)),
    }
}
fn run_prompt() -> Result<(), io::Error> {
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(n) => {
                dbg!(n);
                if n <= 2 {
                    return Ok(());
                }
            }
            Err(msg) => return Err(io::Error::new(io::ErrorKind::Other, msg)),
        }
        println!("> {}", buffer);
        match run(&buffer) {
            Ok(_) => (),
            Err(msg) => return Err(io::Error::new(io::ErrorKind::Other, msg)),
        }
    }
}

fn run(contents: &str) -> Result<(), io::Error> {
    let mut scanner = Lexer::new(contents);
    let tokens = scanner.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlua [script]");
        exit(64)
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("[ERR] {}", msg);
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("[ERR] {}", msg);
                exit(1);
            }
        }
    }
}
