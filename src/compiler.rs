use crate::scanner::{Scanner, Token};

pub fn compile(source: &str) {
    let scanner = Scanner::new(source).skip_ignorable();
    let mut line = 0;
    for token_result in scanner {
        match token_result {
            Ok(Token { span, kind }) => {
                if span.line != line {
                    print!("{:4} ", span.line);
                    line = span.line;
                } else {
                    print!("   | ");
                }
                println!("{:?} '{}'", kind, &source[span.pos..(span.pos + span.len)]);
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
}
