use std::env;
use std::process::exit;

use self::errors::LoxResult;
use self::repl::Repl;
use self::vm::Vm;

mod array;
mod chunk;
mod compiler;
mod errors;
mod repl;
mod scanner;
mod stack;
mod value;
mod vm;

fn run_file(filename: &str) -> LoxResult<()> {
    match std::fs::read_to_string(filename) {
        Ok(source) => Vm::interpret(&source),
        Err(_) => {
            eprintln!("Could not open file \"{filename}\"");
            exit(74);
        }
    }
}

fn main() {
    let mut args = env::args();

    let result = match args.len() {
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

    if let Err(err) = result {
        eprint!("{}", err);
        err.exit();
    }
}
