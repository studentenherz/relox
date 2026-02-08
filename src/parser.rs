use crate::scanner::Token;

pub struct Parser<'a> {
    current: Token<'a>,
    previous: Token<'a>,
    panic_mode: bool,
    had_errors: bool,
}
