use std::mem::MaybeUninit;

use crate::chunk::{Chunk, OpCode};
use crate::scanner::{Scanner, Token, TokenKind};
use crate::value::Value;

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    pub fn one_higher(&self) -> Self {
        match self {
            Self::None => Self::Assignment,
            Self::Assignment => Self::Or,
            Self::Or => Self::And,
            Self::And => Self::Equality,
            Self::Equality => Self::Comparison,
            Self::Comparison => Self::Term,
            Self::Term => Self::Factor,
            Self::Factor => Self::Unary,
            Self::Unary => Self::Call,
            Self::Call | Self::Primary => Self::Primary,
        }
    }
}

impl From<TokenKind> for Precedence {
    fn from(value: TokenKind) -> Self {
        match value {
            TokenKind::LeftParen => Self::None,
            TokenKind::RightParen => Self::None,
            TokenKind::LeftBrace => Self::None,
            TokenKind::RightBrace => Self::None,
            TokenKind::Comma => Self::None,
            TokenKind::Dot => Self::None,
            TokenKind::Minus => Self::Term,
            TokenKind::Plus => Self::Term,
            TokenKind::Semicolon => Self::None,
            TokenKind::Slash => Self::Factor,
            TokenKind::Star => Self::Factor,
            TokenKind::Bang => Self::None,
            TokenKind::BangEqual => Self::None,
            TokenKind::Equal => Self::None,
            TokenKind::EqualEqual => Self::None,
            TokenKind::Greater => Self::None,
            TokenKind::GreaterEqual => Self::None,
            TokenKind::Less => Self::None,
            TokenKind::LessEqual => Self::None,
            TokenKind::Ident => Self::None,
            TokenKind::String => Self::None,
            TokenKind::Number => Self::None,
            TokenKind::And => Self::None,
            TokenKind::Class => Self::None,
            TokenKind::Else => Self::None,
            TokenKind::False => Self::None,
            TokenKind::Fun => Self::None,
            TokenKind::For => Self::None,
            TokenKind::If => Self::None,
            TokenKind::Nil => Self::None,
            TokenKind::Or => Self::None,
            TokenKind::Print => Self::None,
            TokenKind::Super => Self::None,
            TokenKind::Return => Self::None,
            TokenKind::This => Self::None,
            TokenKind::True => Self::None,
            TokenKind::Var => Self::None,
            TokenKind::While => Self::None,
            TokenKind::SingleLineComment => Self::None,
            TokenKind::MultiLineComment => Self::None,
            TokenKind::Whitespace => Self::None,
            TokenKind::Error(_) => Self::None,
            TokenKind::Eof => Self::None,
        }
    }
}

pub struct Parser<'a> {
    source: &'a str,
    scanner: Scanner<'a>,
    current: MaybeUninit<Token>,
    previous: MaybeUninit<Token>,
    panic_mode: bool,
    had_errors: bool,
    chunk: Chunk,
}

pub struct ParserError;

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let scanner = Scanner::new(source).skip_ignorable();
        Self {
            source,
            scanner,
            current: MaybeUninit::uninit(),
            previous: MaybeUninit::uninit(),
            panic_mode: false,
            had_errors: false,
            chunk: Chunk::new(),
        }
    }

    /// # Safety
    ///
    /// Must only be called after `advance()` has been called at least once.
    unsafe fn current(&self) -> &Token {
        unsafe { self.current.assume_init_ref() }
    }

    /// # Safety
    ///
    /// Must only be called after `advance()` has been called at least twice.
    unsafe fn previous(&self) -> &Token {
        unsafe { self.previous.assume_init_ref() }
    }

    /// # Safety
    ///
    /// Requires that `previous()` is valid (advance() has been called twice).
    unsafe fn emit_byte(&mut self, byte: impl Into<u8>) {
        unsafe {
            self.chunk.write(byte, self.previous().span.line);
        }
    }

    /// # Safety
    ///
    /// Requires that `previous()` is valid (advance() has been called twice).
    unsafe fn emit_return(&mut self) {
        unsafe {
            self.emit_byte(OpCode::Return);
        }
    }

    /// # Safety
    ///
    /// Requires that `previous()` is valid (advance() has been called twice).
    unsafe fn emit_constant(&mut self, value: Value) {
        unsafe {
            self.chunk.write_constant(value, self.previous().span.line);
        }
    }

    fn error_at(&mut self, token: Token, message: &str) {
        eprint!("[line {}] Error", token.span.line);

        if token.kind == TokenKind::Eof {
            eprint!(" at end");
        } else if matches!(token.kind, TokenKind::Error(_)) {
            // Nothing.
        } else {
            eprint!(" at '{}'", token.slice(self.source));
        }

        eprintln!(": {}", message);
        self.had_errors = true;
        self.panic_mode = true;
    }

    /// # Safety
    ///
    /// Requires that `current()` is valid (advance() has been called).
    unsafe fn error_at_current(&mut self, message: &str) {
        unsafe {
            self.error_at(*self.current(), message);
        }
    }

    /// # Safety
    ///
    /// Requires that `previous()` is valid (advance() has been called twice).
    unsafe fn error(&mut self, message: &str) {
        unsafe {
            self.error_at(*self.previous(), message);
        }
    }

    /// # Safety
    ///
    /// Requires that `previous()` is valid (advance() has been called twice).
    unsafe fn number(&mut self) {
        unsafe {
            let number = self
                .previous()
                .slice(self.source)
                .parse::<Value>()
                .expect("Error parsing a number");
            self.emit_constant(number);
        }
    }

    /// # Safety
    ///
    /// Requires that tokens are initialized (advance() has been called twice).
    unsafe fn grouping(&mut self) {
        unsafe {
            self.expression();
            self.consume(TokenKind::RightParen, "Expect ')' after expression.");
        }
    }

    /// # Safety
    ///
    /// Requires that `previous()` is valid (advance() has been called twice).
    unsafe fn unary(&mut self) {
        unsafe {
            let operation_kind = self.previous().kind;

            self.parse_precedence(Precedence::Unary);

            match operation_kind {
                TokenKind::Minus => self.emit_byte(OpCode::Negate),
                _ => unreachable!(),
            }
        }
    }

    /// # Safety
    ///
    /// Requires that `previous()` is valid (advance() has been called twice).
    unsafe fn binary(&mut self) {
        unsafe {
            let operation_kind = self.previous().kind;
            let precedence: Precedence = operation_kind.into();
            self.parse_precedence(precedence.one_higher());

            match operation_kind {
                TokenKind::Plus => self.emit_byte(OpCode::Add),
                TokenKind::Minus => self.emit_byte(OpCode::Subtract),
                TokenKind::Star => self.emit_byte(OpCode::Multiply),
                TokenKind::Slash => self.emit_byte(OpCode::Divide),
                _ => unreachable!(),
            }
        }
    }

    /// # Safety
    ///
    /// Requires that `previous()` is valid (advance() has been called twice).
    unsafe fn prefix(&mut self) {
        unsafe {
            match self.previous().kind {
                TokenKind::LeftParen => self.grouping(),
                TokenKind::Minus => self.unary(),
                TokenKind::Number => self.number(),
                _ => self.error("Expect expression."),
            }
        }
    }

    /// # Safety
    ///
    /// Requires that `previous()` is valid (advance() has been called twice).
    unsafe fn infix(&mut self) {
        unsafe {
            match self.previous().kind {
                TokenKind::Minus | TokenKind::Plus | TokenKind::Star | TokenKind::Slash => {
                    self.binary()
                }
                _ => self.error("Expect expression."),
            }
        }
    }

    /// # Safety
    ///
    /// Requires that `prefix()` is valid. This function calls `advance()` once before
    /// `prefix()`. `prefix()` needs two calls to `advance()` to be valid, then, this
    /// funciton needs `advance()` has been called to be valid.
    unsafe fn parse_precedence(&mut self, precedence: Precedence) {
        unsafe {
            self.advance();
            self.prefix();

            while precedence <= self.current().kind.into() {
                self.advance();
                self.infix();
            }
        }
    }

    /// Parse the source code and return the compiled chunk.
    pub fn parse(&mut self) -> Result<Chunk, ParserError> {
        // This is the main entry point for parsing. It establishes the invariant
        // that current and previous are initialized, then all internal unsafe
        // functions can rely on this invariant.

        // First call initializes `current`
        self.advance();

        // SAFETY: advance() was called, so current is initialized. `previous()`
        // calls `advance()` at least once more, thus stablishing the invariance
        // that both current and previous are initialized.
        unsafe {
            self.expression();
            self.consume(TokenKind::Eof, "Expect end of expression.");
        }

        if !self.had_errors {
            // SAFETY: advance() was called, invariant still holds
            unsafe {
                self.emit_return();
            }
            return Ok(std::mem::replace(&mut self.chunk, Chunk::new()));
        }

        Err(ParserError)
    }

    /// # Safety
    ///
    /// Requires that `parse_precedence()` is valid (advance() has been called).
    unsafe fn expression(&mut self) {
        unsafe {
            self.parse_precedence(Precedence::Assignment);
        }
    }

    /// Advance to the next token.
    ///
    /// The first call to this function initializes the `current`, the second
    /// call initializes `previous`. Thus, two calls to this function are needed
    /// to stablish the invariant that both `current` and `previous` are initialized.
    pub fn advance(&mut self) {
        loop {
            match self.scanner.next() {
                Some(
                    token @ Token {
                        kind: TokenKind::Error(message),
                        ..
                    },
                ) => {
                    self.error_at(token, message);
                }
                Some(token) => {
                    self.previous = std::mem::replace(&mut self.current, MaybeUninit::new(token));
                    return;
                }
                None => {
                    self.previous =
                        std::mem::replace(&mut self.current, MaybeUninit::new(self.scanner.eof()));
                    return;
                }
            }
        }
    }

    /// # Safety
    ///
    /// Requires that `current()` is valid (advance() has been called).
    unsafe fn consume(&mut self, kind: TokenKind, message: &str) {
        unsafe {
            if self.current().kind == kind {
                self.advance();
                return;
            }

            self.error_at_current(message);
        }
    }
}
