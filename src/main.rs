use std::env;
use std::process::exit;

use self::interpreter::Interpreter;
use self::repl::Repl;

mod array;
mod chunk;
mod compiler;
mod errors;
mod interpreter;
mod parser;
mod repl;
mod scanner;
mod span;
mod stack;
mod value;
mod vm;

fn run_file(filename: &str) {
    let mut interpreter = Interpreter::new();
    match std::fs::read_to_string(filename) {
        Ok(source) => {
            if let Err(err) = interpreter.interpret(&source) {
                err.exit();
            }
        }
        Err(_) => {
            eprintln!("Could not open file \"{filename}\"");
            exit(74);
        }
    }
}

fn main() {
    let mut args = env::args();

    match args.len() {
        1 => {
            let mut repl = Repl::new();
            repl.run()
        }
        2 => {
            let filename = unsafe { args.nth(1).unwrap_unchecked() };
            run_file(&filename)
        }
        _ => {
            println!("Usage: relox [path]");
            exit(64)
        }
    };
}
