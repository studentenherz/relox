use std::io::Write;

use crate::errors::LoxResult;
use crate::vm::Vm;

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

    fn print_prompt() -> LoxResult<()> {
        print!("> ");
        std::io::stdout().flush()?;

        Ok(())
    }

    fn read_input() -> LoxResult<Option<String>> {
        let mut buffer = String::new();
        if std::io::stdin().read_line(&mut buffer)? == 0 {
            return Ok(None);
        }
        Ok(Some(buffer.trim().to_string()))
    }

    pub fn run(&mut self) -> LoxResult<()> {
        println!("{}", Self::welcome_message());
        loop {
            Self::print_prompt()?;
            if let Some(input) = Self::read_input()? {
                Vm::interpret(&input)?;
            } else {
                println!();
                return Ok(());
            }
        }
    }
}
