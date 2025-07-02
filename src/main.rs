use std::process::exit;
use std::{env, fs, io};

fn run_file(file_path: &str) -> Result<(), io::Error> {
    match fs::read_to_string(file_path) {
        Ok(content) => return run(&content),
        Err(msg) => return Err(io::Error::new(io::ErrorKind::Other, msg)),
    }
}
fn run_prompt() -> Result<(), io::Error> {
    print!("> ");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    println!("You wrote: {}", buffer);
    Ok(())
}

fn run(_contents: &str) -> Result<(), io::Error> {
    Err(io::Error::new(io::ErrorKind::Other, "Not Implemeted"))
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
