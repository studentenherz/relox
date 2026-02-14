use std::str::Chars;

use crate::span::Span;
use macros::DebugC;

#[derive(Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(DebugC, PartialEq, Clone, Copy)]
#[prefix = "TOKEN"]
pub enum TokenKind {
    // Singe-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literlas
    Ident,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Super,
    Return,
    This,
    True,
    Var,
    While,

    // Ignorable
    SingleLineComment,
    MultiLineComment,
    Whitespace,

    Error(&'static str),
    Eof,
}

const EOF_CHAR: char = '\0';

pub struct Scanner<'a> {
    iter: Chars<'a>,
    line: usize,
    prev: char,
    col: usize,
    pos: usize,
    start_line: usize,
    start_pos: usize,
    start_col: usize,
    skip_ignorable: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        let iter = input.chars();
        Self {
            iter,
            prev: EOF_CHAR,
            line: 1,
            col: 1,
            pos: 0,
            start_line: 1,
            start_col: 1,
            start_pos: 0,
            skip_ignorable: false,
        }
    }

    pub fn skip_ignorable(self) -> Self {
        Self {
            skip_ignorable: true,
            ..self
        }
    }

    fn bump(&mut self) {
        self.col += 1;
        self.pos += 1;

        let _next = self.iter.next();
        self.prev = _next.unwrap_or(EOF_CHAR);
        if self.prev == '\n' {
            self.line += 1;
            self.col = 1;
        }
    }

    fn peek_first(&mut self) -> Option<char> {
        self.iter.clone().next()
    }

    fn peek_second(&mut self) -> Option<char> {
        let mut iter = self.iter.clone();
        iter.next();
        iter.next()
    }

    fn second_matches(&mut self, expected: char) -> bool {
        if let Some(second_char) = self.peek_second()
            && second_char == expected
        {
            return true;
        }

        false
    }

    fn span(&self) -> Span {
        Span {
            // col: self.start_col,
            line: self.line,
            start: self.start_pos,
            lenth: self.pos - self.start_pos,
        }
    }

    fn error(&self, reason: &'static str) -> Token {
        Token {
            kind: TokenKind::Error(reason),
            span: self.span(),
        }
    }

    pub fn eof(&self) -> Token {
        Token {
            kind: TokenKind::Eof,
            span: self.span(),
        }
    }

    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_alphanumeric(c: char) -> bool {
        Self::is_alpha(c) || c.is_ascii_digit()
    }

    fn comment_or_slash(&mut self) -> Result<TokenKind, Token> {
        if self.second_matches('/') {
            self.bump();
            self.bump();
            self.eat_while(|c| c != '\n');
            Ok(TokenKind::SingleLineComment)
        } else if self.second_matches('*') {
            self.bump();
            self.bump();
            let mut openning_comments = 1usize;
            let mut comment = String::new();
            if let Some(mut first_char) = self.peek_first() {
                while let Some(second_char) = self.peek_second() {
                    if first_char == '/' && second_char == '*' {
                        openning_comments += 1;
                    }
                    if first_char == '*' && second_char == '/' {
                        self.bump();
                        self.bump();
                        openning_comments -= 1;
                        if openning_comments == 0 {
                            break;
                        }
                    }

                    comment.push(first_char);
                    first_char = second_char;
                    self.bump();
                }
            }

            if openning_comments == 0 {
                Ok(TokenKind::MultiLineComment)
            } else {
                Err(self.error("Expect closing '*/' for multiline comment."))
            }
        } else {
            self.bump();
            Ok(TokenKind::Slash)
        }
    }

    fn check_word(&mut self, rest: &'static str, kind: TokenKind) -> TokenKind {
        self.bump();

        for char in rest.chars() {
            match self.peek_first() {
                Some(c) if c == char => {
                    self.bump();
                }
                _ => {
                    return self.ident();
                }
            }
        }

        if let Some(c) = self.peek_first()
            && c.is_alphanumeric()
        {
            return self.ident();
        }

        kind
    }

    fn ident(&mut self) -> TokenKind {
        self.eat_while(Self::is_alphanumeric);
        TokenKind::Ident
    }

    fn keyword_or_ident(&mut self) -> TokenKind {
        match unsafe { self.peek_first().unwrap_unchecked() } {
            'a' => self.check_word("nd", TokenKind::And),
            'c' => self.check_word("lass", TokenKind::Class),
            'e' => self.check_word("lse", TokenKind::Else),
            'f' => {
                self.bump();
                match self.peek_first() {
                    Some('a') => self.check_word("lse", TokenKind::False),
                    Some('o') => self.check_word("r", TokenKind::For),
                    Some('u') => self.check_word("n", TokenKind::Fun),
                    _ => self.ident(),
                }
            }
            'i' => self.check_word("f", TokenKind::If),
            'n' => self.check_word("il", TokenKind::Nil),
            'o' => self.check_word("r", TokenKind::Or),
            'p' => self.check_word("rint", TokenKind::Print),
            'r' => self.check_word("eturn", TokenKind::Return),
            's' => self.check_word("uper", TokenKind::Super),
            't' => {
                self.bump();
                match self.peek_first() {
                    Some('h') => self.check_word("is", TokenKind::This),
                    Some('r') => self.check_word("ue", TokenKind::True),
                    _ => self.ident(),
                }
            }
            'v' => self.check_word("ar", TokenKind::Var),
            'w' => self.check_word("hile", TokenKind::While),
            _ => self.ident(),
        }
    }

    fn number(&mut self) -> TokenKind {
        let mut has_dot = false;
        let mut number = String::new();

        while let Some(c) = self.peek_first() {
            if c.is_ascii_digit() {
                number.push(c);
                self.bump();
                continue;
            }

            if c == '.'
                && !has_dot
                && let Some(c2) = self.peek_second()
                && c2.is_ascii_digit()
            {
                has_dot = true;
                number.push(c);
                self.bump();
                continue;
            }

            break;
        }

        TokenKind::Number
    }

    fn string(&mut self) -> Result<TokenKind, Token> {
        self.bump();
        let mut escaped = false;
        self.eat_while(move |c| {
            let cont = escaped || c != '"';
            escaped = c == '\\';
            cont
        });

        if self.peek_first() != Some('"') {
            return Err(self.error("Unterminated string."));
        }

        self.bump();
        Ok(TokenKind::String)
    }

    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while let Some(second_char) = self.peek_first() {
            if !predicate(second_char) {
                break;
            }
            self.bump();
        }
    }

    fn advance_token(&mut self, first_char: char) -> Result<Token, Token> {
        self.start_pos = self.pos;
        self.start_col = self.col;
        self.start_line = self.line;

        let token = match first_char {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ';' => TokenKind::Semicolon,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            '-' => TokenKind::Minus,
            '+' => TokenKind::Plus,
            '*' => TokenKind::Star,
            '!' => {
                if self.second_matches('=') {
                    self.bump();
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }
            '=' => {
                if self.second_matches('=') {
                    self.bump();
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }
            '<' => {
                if self.second_matches('=') {
                    self.bump();
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            '>' => {
                if self.second_matches('=') {
                    self.bump();
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            _ => TokenKind::Error("Unknown token."),
        };

        let token_kind = match token {
            TokenKind::Error(_) => match first_char {
                '"' => self.string()?,
                '/' => self.comment_or_slash()?,
                c if c.is_ascii_digit() => self.number(),
                c if Self::is_alpha(c) => self.keyword_or_ident(),
                c if c.is_ascii_whitespace() => {
                    self.eat_while(|c| c.is_ascii_whitespace());
                    TokenKind::Whitespace
                }
                _ => {
                    self.bump();
                    token
                }
            },
            _ => {
                self.bump();
                token
            }
        };

        Ok(Token {
            kind: token_kind,
            span: self.span(),
        })
    }
}

impl Iterator for Scanner<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.peek_first() {
                None => return None,
                Some(first_char) => {
                    let token_result = self.advance_token(first_char);
                    if let Ok(token) = &token_result
                        && self.skip_ignorable
                        && matches!(
                            token.kind,
                            TokenKind::Whitespace
                                | TokenKind::MultiLineComment
                                | TokenKind::SingleLineComment
                        )
                    {
                        continue;
                    }
                    return Some(match token_result {
                        Ok(token) => token,
                        Err(token) => token,
                    });
                }
            }
        }
    }
}

impl<'a> Token {
    pub fn slice(&self, source: &'a str) -> &'a str {
        &source[self.span.start..(self.span.start + self.span.lenth)]
    }
}
