use std::{
    error::Error,
    io::{self, Write},
};

use monkey_rs::Lex;

const PROMPT: &str = ">> ";

type MyResult = Result<(), Box<dyn Error>>;

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!",
        whoami::username()
    );
    println!("Feel free to type in commands");
    if let Err(e) = start() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn start() -> MyResult {
    let mut input = String::new();
    loop {
        print!("{PROMPT}");
        io::stdout().flush()?;

        io::stdin().read_line(&mut input)?;
        for token in input.tokens() {
            println!("{:?}", token);
        }
        input.clear();
    }
}
