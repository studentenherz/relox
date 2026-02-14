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

    fn current(&self) -> &Token {
        unsafe { &self.current.assume_init_ref() }
    }

    fn previous(&self) -> &Token {
        unsafe { &self.previous.assume_init_ref() }
    }

    fn emit_byte(&mut self, byte: impl Into<u8>) {
        self.chunk.write(byte, self.previous().span.line);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return);
    }

    fn emit_bytes(&mut self, byte1: impl Into<u8>, byte2: impl Into<u8>) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_constant(&mut self, value: Value) {
        self.chunk.write_constant(value, self.previous().span.line);
    }

    fn error_at(&mut self, token: Token, message: &str) {
        eprint!("[line {}] Error", token.span.line);

        if token.kind == TokenKind::Eof {
            eprint!(" at end");
        } else if matches!(token.kind, TokenKind::Error(_)) {
            // Nothig.
        } else {
            eprint!(" at '{}'", token.slice(&self.source));
        }

        eprintln!(": {}", message);
        self.had_errors = true;
        self.panic_mode = true;
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(*self.current(), message);
    }

    fn error(&mut self, message: &str) {
        self.error_at(*self.previous(), message);
    }

    fn number(&mut self) {
        let number = self
            .previous()
            .slice(self.source)
            .parse::<Value>()
            .expect("Error parsing a number");
        self.emit_constant(number);
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenKind::RightParen, "Expect ')' after expression.");
    }

    fn unary(&mut self) {
        let operation_kind = self.previous().kind;

        self.parse_precedence(Precedence::Unary);

        match operation_kind {
            TokenKind::Minus => self.emit_byte(OpCode::Negate),
            _ => unreachable!(),
        }
    }

    fn binary(&mut self) {
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

    fn prefix(&mut self) {
        match self.previous().kind {
            TokenKind::LeftParen => self.grouping(),
            TokenKind::Minus => self.unary(),
            TokenKind::Number => self.number(),
            _ => self.error("Expect expression."),
        }
    }

    fn infix(&mut self) {
        match self.previous().kind {
            TokenKind::Minus | TokenKind::Plus | TokenKind::Star | TokenKind::Slash => {
                self.binary()
            }
            _ => self.error("Expect expression."),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        self.prefix();

        while precedence <= self.current().kind.into() {
            self.advance();
            self.infix();
        }
    }

    pub fn parse(&mut self) -> Result<Chunk, ParserError> {
        self.advance();
        self.expression();
        self.consume(TokenKind::Eof, "Expect end of expression.");

        if !self.had_errors {
            self.emit_return();
            return Ok(std::mem::replace(&mut self.chunk, Chunk::new()));
        }

        Err(ParserError)
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

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

    fn consume(&mut self, kind: TokenKind, message: &str) {
        if self.current().kind == kind {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }
}
