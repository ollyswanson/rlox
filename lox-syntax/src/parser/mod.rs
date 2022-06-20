use std::borrow::Cow;
use std::mem;

use error::{PResult, ParseError};
use scanner::Scanner;

use crate::ast::stmt::Stmt;
use crate::ast::IdentifierId;
use crate::span::Span;
use crate::token::{Token, TokenKind};

pub mod error;
mod expr;
mod scanner;
mod stmt;

//  A unique id for the Identifiers is needed for the following case:
//
// var a = 1;
// fun() {
//   print a;
//   var a = 2;
//   print a;
//
// Without a unique id and just using the name, during resolution `a` would first be pointed at the
// `a` in the global scope, and would then be pointed at the `a` in the function scope, such that
// instead of `1` and `2` being printed to stdout, there would be an error due to printing an
// undeclared variable.
#[derive(Debug, Copy, Clone, Default)]
pub struct ParserState {
    variable_id: IdentifierId,
}

impl ParserState {
    pub fn new() -> Self {
        Self { variable_id: 0 }
    }
}

pub struct Parser<'a> {
    state: ParserState,
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
            state: ParserState::new(),
            scanner,
            current_token,
            // Use EOF token as dummy to start the scanner
            prev_token: Token::new(TokenKind::Eof, Span::new(0, 0)),
            diagnostics: Vec::new(),
        }
    }

    /// Used in REPL mode. A new parser is instantiated for each line of input for simplicity.
    /// However each usage of a variable is assigned a unique id, therefore state needs to be passed
    /// between parsers to allow continuation without using global state.
    pub fn with_state(mut self, state: ParserState) -> Self {
        self.state = state;
        self
    }

    pub fn state(&self) -> ParserState {
        self.state
    }

    pub fn diagnostics(&self) -> &[ParseError] {
        &self.diagnostics
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.parse_declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    self.diagnostics.push(e);
                    self.synchronize();
                }
            }
        }

        statements
    }

    fn bump(&mut self) -> &Token {
        mem::swap(&mut self.prev_token, &mut self.current_token);
        self.current_token = self.scanner.next().expect("Should not advance past EOF");
        self.prev()
    }

    fn synchronize(&mut self) {
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

    fn peek(&self) -> &Token {
        &self.current_token
    }

    fn prev(&self) -> &Token {
        &self.prev_token
    }

    fn is_at_end(&self) -> bool {
        self.current_token.kind == TokenKind::Eof
    }

    fn matches(&mut self, kinds: &[TokenKind]) -> Option<&Token> {
        let token = self.peek();

        if kinds.iter().any(|kind| token.kind.match_kind(kind)) {
            Some(self.bump())
        } else {
            None
        }
    }

    fn match_and_or<T, F>(&mut self, kinds: &[TokenKind], and: T, or: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        if self.matches(kinds).is_some() {
            and
        } else {
            or(self)
        }
    }

    fn peek_and_or<T, F>(&mut self, kinds: &[TokenKind], and: T, or: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        if kinds
            .iter()
            .any(|kind| kind.match_kind(&self.current_token.kind))
        {
            and
        } else {
            or(self)
        }
    }

    fn expect(&mut self, expected: TokenKind, message: Cow<'static, str>) -> PResult<&Token> {
        if self.peek().kind.match_kind(&expected) {
            Ok(self.bump())
        } else {
            let token = self.peek().clone();
            Err(ParseError::UnexpectedToken {
                message,
                span: token.span,
                kind: token.kind,
            })
        }
    }

    fn increment(&mut self) -> usize {
        let next_id = self.state.variable_id + 1;
        std::mem::replace(&mut self.state.variable_id, next_id)
    }
}
