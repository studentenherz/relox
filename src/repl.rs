use std::io::Write;

use crate::interpreter::Interpreter;

pub struct Repl {}

impl Repl {
    pub fn new() -> Self {
        Self {}
    }

    fn welcome_message() -> String {
        let version = env!("CARGO_PKG_VERSION");
        let name = env!("CARGO_PKG_NAME");
        let authors = env!("CARGO_PKG_AUTHORS");
        let quit = "^D";
        format!("Welcome to {name} version {version} by {authors}\nUse {quit} to quit")
    }

    fn print_prompt() {
        print!("> ");
        std::io::stdout()
            .flush()
            .expect("Couldn't flush the standard output");
    }

    fn read_input() -> Option<String> {
        let mut buffer = String::new();
        if std::io::stdin()
            .read_line(&mut buffer)
            .expect("Couldn't read from the standard input")
            == 0
        {
            return None;
        }
        Some(buffer.trim().to_string())
    }

    pub fn run(&mut self) {
        let mut interpreter = Interpreter::new();
        println!("{}", Self::welcome_message());
        loop {
            Self::print_prompt();
            if let Some(input) = Self::read_input() {
                let _ = interpreter.interpret(&input);
            } else {
                println!();
                return;
            }
        }
    }
}
