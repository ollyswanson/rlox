use std::borrow::Cow;
use std::mem;

use error::{PResult, ParseError};
use scanner::Scanner;

use crate::span::Span;
use crate::token::{Token, TokenKind};

pub mod error;
mod expr;
mod scanner;

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    current_token: Token,
    prev_token: Token,
    diagnostics: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut scanner = Scanner::new(source);
        // Safe to unwrap as there should always be at least an EOF token
        let current_token = scanner.next().unwrap();

        Self {
            scanner,
            current_token,
            // Use EOF token as dummy to start the scanner
            prev_token: Token::new(TokenKind::Eof, Span::new(0, 0)),
            diagnostics: Vec::new(),
        }
    }

    pub fn bump(&mut self) -> &Token {
        mem::swap(&mut self.prev_token, &mut self.current_token);
        self.current_token = self.scanner.next().expect("Should not advance past EOF");
        self.prev()
    }

    pub fn synchronize(&mut self) {
        use TokenKind::*;
        while !self.is_at_end() {
            let token = self.bump();
            let span = token.span;

            match token.kind {
                Semicolon => {
                    return;
                }
                Error(error) => {
                    self.diagnostics.push(ParseError::ScanError { error, span });
                }
                _ => {
                    if matches!(
                        self.peek().kind,
                        Class | For | Fun | If | Print | Return | Var | While
                    ) {
                        break;
                    } else {
                        continue;
                    }
                }
            }
        }
    }

    #[inline]
    pub fn peek(&self) -> &Token {
        &self.current_token
    }

    #[inline]
    pub fn prev(&self) -> &Token {
        &self.prev_token
    }

    #[inline]
    pub fn is_at_end(&self) -> bool {
        self.current_token.kind == TokenKind::Eof
    }

    #[inline]
    pub fn matches(&mut self, kinds: &[TokenKind]) -> Option<&Token> {
        let token = self.peek();

        if kinds.iter().any(|kind| token.kind.match_kind(kind)) {
            Some(self.bump())
        } else {
            None
        }
    }

    pub fn expect(&mut self, expected: TokenKind, message: Cow<'static, str>) -> PResult<&Token> {
        if self.peek().kind.match_kind(&expected) {
            Ok(self.bump())
        } else {
            Err(ParseError::UnexpectedToken {
                message,
                span: self.peek().span,
            })
        }
    }
}
