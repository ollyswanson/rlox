use std::iter::{Enumerate, Peekable};
use std::str::Chars;

use crate::span::Span;
use crate::token;
use crate::token::{ScanError, Token, TokenKind};

static KEYWORDS: phf::Map<&'static str, TokenKind> = phf::phf_map! {
    "and" => TokenKind::And,
    "class" => TokenKind::Class,
    "else" => TokenKind::Else,
    "false" => TokenKind::False,
    "fun" => TokenKind::Fun,
    "for" => TokenKind::For,
    "if" => TokenKind::If,
    "nil" => TokenKind::Nil,
    "or" => TokenKind::Or,
    "print" => TokenKind::Print,
    "return" =>TokenKind::Return,
    "super" => TokenKind::Super,
    "this" => TokenKind::This,
    "true" => TokenKind::True,
    "var" => TokenKind::Var,
    "while" => TokenKind::While,
};

struct ScannerInner<'a> {
    /// The source
    source: &'a str,
    /// An iterator over the chars in the source.
    source_iter: Peekable<Enumerate<Chars<'a>>>,
    /// The start of the next lexeme
    start: usize,
    /// Has reached EOF
    eof: bool,
}

impl ScannerInner<'_> {
    fn scan_token(&mut self) -> Token {
        use TokenKind::*;

        if let Some((start, c)) = self.source_iter.next() {
            self.start = start;
            match c {
                // Ignore whitespace
                ' ' | '\n' | '\r' | '\t' => self.scan_token(),
                // Single character tokens,
                '(' => token!(LeftParen, start, 1),
                ')' => token!(RightParen, start, 1),
                '{' => token!(LeftBrace, start, 1),
                '}' => token!(RightBrace, start, 1),
                ',' => token!(Comma, start, 1),
                '.' => token!(Dot, start, 1),
                '-' => token!(Minus, start, 1),
                '+' => token!(Plus, start, 1),
                ';' => token!(Semicolon, start, 1),
                '*' => token!(Star, start, 1),
                '/' => {
                    if self.peek().map_or(false, |c| c == '/') {
                        self.take_until('\n');
                        self.scan_token()
                    } else {
                        token!(Slash, start, 1)
                    }
                }
                '!' => self.take_select('=', token!(BangEqual, start, 2), token!(Bang, start, 1)),
                '>' => self.take_select(
                    '=',
                    token!(GreaterEqual, start, 2),
                    token!(Greater, start, 1),
                ),
                '<' => self.take_select('=', token!(LessEqual, start, 2), token!(Less, start, 1)),
                '=' => self.take_select('=', token!(EqualEqual, start, 2), token!(Equal, start, 1)),
                '"' => self.string(),
                c if c.is_digit(10) => self.number(),
                c if c.is_alphabetic() || c == '_' => self.identifier(),
                unrecognized => token!(
                    Error(ScanError::UnrecognizedToken { unrecognized }),
                    start,
                    1
                ),
            }
        } else {
            self.eof = true;
            token!(Eof, self.source.len(), 0)
        }
    }

    fn string(&mut self) -> Token {
        self.take_until('"');
        if let Some(close) = self.source_iter.next() {
            let string = self.source[self.start + 1..close.0].to_owned();

            Token::new(
                TokenKind::String(string),
                Span::new(self.start, close.0 + 1),
            )
        } else {
            Token::new(
                TokenKind::Error(ScanError::UnterminatedString),
                Span::new(self.start, self.source.len() - 1),
            )
        }
    }

    fn number(&mut self) -> Token {
        self.take_while(char::is_numeric);

        match (self.peek(), self.peek_nth(1)) {
            (Some('.'), Some(n)) if n.is_numeric() => {
                self.source_iter.next();
                self.take_while(|c| c.is_digit(10))
            }
            _ => {}
        }

        // safe to unwrap as slice will be of form "x" | "x.y"
        let number: f64 = dbg!(&self.source[self.start..self.pos()]).parse().unwrap();

        Token::new(
            TokenKind::Number(dbg!(number)),
            Span::new(self.start, self.pos()),
        )
    }

    fn identifier(&mut self) -> Token {
        self.take_while(|c| c.is_alphanumeric() || c == '_');

        let identifier = &self.source[self.start..self.pos()];

        if let Some(identifier) = KEYWORDS.get(identifier) {
            Token::new(identifier.clone(), Span::new(self.start, self.pos()))
        } else {
            Token::new(
                TokenKind::Identifier(identifier.to_owned()),
                Span::new(self.start, self.pos()),
            )
        }
    }
}

impl<'a> ScannerInner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            source_iter: source.chars().enumerate().peekable(),
            start: 0,
            eof: false,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.source_iter.peek().map(|(_i, c)| *c)
    }

    fn peek_nth(&self, n: usize) -> Option<char> {
        let mut temp_iter = self.source_iter.clone();
        temp_iter.nth(n).map(|(_i, c)| c)
    }

    fn pos(&mut self) -> usize {
        match self.source_iter.peek().map(|(i, _c)| *i) {
            Some(i) => i,
            None => self.source.len(),
        }
    }

    /// Consumes the next character if it matches `expected` and returns `true` else `false`.
    fn take(&mut self, expected: char) -> bool {
        self.source_iter.next_if(|(_, c)| *c == expected).is_some()
    }

    /// If the next char is `expected` then consume and return `a` else return `b`.
    fn take_select<T>(&mut self, expected: char, a: T, b: T) -> T {
        match self.take(expected) {
            true => a,
            _ => b,
        }
    }

    fn take_while<P>(&mut self, pred: P)
    where
        P: Fn(char) -> bool,
    {
        while self.source_iter.next_if(|(_, c)| pred(*c)).is_some() {}
    }

    /// Consumes `source_iter` until `until` is encountered
    fn take_until(&mut self, until: char) {
        while self.source_iter.next_if(|(_, c)| *c != until).is_some() {}
    }
}

pub struct Scanner<'a> {
    inner: ScannerInner<'a>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            inner: ScannerInner::new(source),
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.eof {
            None
        } else {
            Some(self.inner.scan_token())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(source: &str) -> Vec<Token> {
        Scanner::new(source).collect()
    }

    macro_rules! compare_tokens {
        ($expected:ident, $actual:ident) => {
            assert_eq!($expected.len(), $actual.len());

            for (expected, actual) in $expected.into_iter().zip($actual.into_iter()) {
                assert_eq!(expected, actual);
            }
        };
    }

    #[test]
    fn tokenizes_single_character_tokens() {
        use TokenKind::*;
        let source = "(){},.-+;/*";
        let expected = vec![
            token!(LeftParen, 0, 1),
            token!(RightParen, 1, 1),
            token!(LeftBrace, 2, 1),
            token!(RightBrace, 3, 1),
            token!(Comma, 4, 1),
            token!(Dot, 5, 1),
            token!(Minus, 6, 1),
            token!(Plus, 7, 1),
            token!(Semicolon, 8, 1),
            token!(Slash, 9, 1),
            token!(Star, 10, 1),
            token!(Eof, 11, 0),
        ];

        let tokens = tokenize(source);
        compare_tokens!(expected, tokens);
    }

    #[test]
    fn tokenizes_one_or_two_character_operators() {
        use TokenKind::*;
        let source = "!!====<<=>>=";
        let expected = vec![
            token!(Bang, 0, 1),
            token!(BangEqual, 1, 2),
            token!(EqualEqual, 3, 2),
            token!(Equal, 5, 1),
            token!(Less, 6, 1),
            token!(LessEqual, 7, 2),
            token!(Greater, 9, 1),
            token!(GreaterEqual, 10, 2),
            token!(Eof, 12, 0),
        ];

        let tokens = tokenize(source);

        compare_tokens!(expected, tokens);
    }

    #[test]
    fn ignores_comments() {
        use TokenKind::*;
        let source = r#"
        // This is a comment
        "#;
        let expected = vec![token!(Eof, 38, 0)];

        let tokens = tokenize(source);

        compare_tokens!(expected, tokens);
    }

    #[test]
    fn tokenizes_string_literals() {
        let source = r#"
        "This is a string literal.
        It is on many lines"
        "#;

        let expected = vec![
            token!(
                TokenKind::String(
                    r#"This is a string literal.
        It is on many lines"#
                        .to_owned()
                ),
                9,
                55
            ),
            token!(TokenKind::Eof, 73, 0),
        ];

        let tokens = tokenize(source);

        compare_tokens!(expected, tokens);
    }

    #[test]
    fn produces_an_error_when_there_is_an_unterminated_string() {
        use TokenKind::*;

        let source = r#""This is an unterminated string"#;
        let expected = vec![
            token!(Error(ScanError::UnterminatedString), 0, 30),
            token!(Eof, 31, 0),
        ];

        let tokens = tokenize(source);

        compare_tokens!(expected, tokens);
    }

    #[test]
    fn it_tokenizes_number_literals() {
        use TokenKind::*;
        let source = r#"
        1
        1.0
        1.
        11.343
        "#;

        let expected = vec![
            token!(Number(1.0), 9, 1),
            token!(Number(1.0), 19, 3),
            token!(Number(1.0), 31, 1),
            token!(Dot, 32, 1),
            token!(Number(11.343), 42, 6),
            token!(Eof, 57, 0),
        ];

        let tokens = tokenize(source);

        compare_tokens!(expected, tokens);
    }

    #[test]
    fn it_tokenizes_identifiers_and_keywords() {
        use TokenKind::*;
        let source = r#"
        or
        orchid
        _orchid_123_1
        if else
        "#;

        let expected = vec![
            token!(Or, 9, 2),
            token!(Identifier("orchid".to_owned()), 20, 6),
            token!(Identifier("_orchid_123_1".to_owned()), 35, 13),
            token!(If, 57, 2),
            token!(Else, 60, 4),
            token!(Eof, 73, 0),
        ];

        let tokens = tokenize(source);

        compare_tokens!(expected, tokens);
    }

    #[test]
    fn it_can_tokenize_a_code_snippet() {
        use TokenKind::*;

        let source = r#"
        // This is a snippet of Lox code
        class Point {
            init(x, y) {
                this.x = x;
                this.y = y;
                this.z = 1.4;
                this.name = "Pointy McPointface";
            }
        }
        "#;

        let expected = vec![
            token!(Class, 50, 5),
            token!(Identifier("Point".to_owned()), 56, 5),
            token!(LeftBrace, 62, 1),
            token!(Identifier("init".to_owned()), 76, 4),
            token!(LeftParen, 80, 1),
            token!(Identifier("x".to_owned()), 81, 1),
            token!(Comma, 82, 1),
            token!(Identifier("y".to_owned()), 84, 1),
            token!(RightParen, 85, 1),
            token!(LeftBrace, 87, 1),
            token!(This, 105, 4),
            token!(Dot, 109, 1),
            token!(Identifier("x".to_owned()), 110, 1),
            token!(Equal, 112, 1),
            token!(Identifier("x".to_owned()), 114, 1),
            token!(Semicolon, 115, 1),
            token!(This, 133, 4),
            token!(Dot, 137, 1),
            token!(Identifier("y".to_owned()), 138, 1),
            token!(Equal, 140, 1),
            token!(Identifier("y".to_owned()), 142, 1),
            token!(Semicolon, 143, 1),
            token!(This, 161, 4),
            token!(Dot, 165, 1),
            token!(Identifier("z".to_owned()), 166, 1),
            token!(Equal, 168, 1),
            token!(Number(1.4), 170, 3),
            token!(Semicolon, 173, 1),
            token!(This, 191, 4),
            token!(Dot, 195, 1),
            token!(Identifier("name".to_owned()), 196, 4),
            token!(Equal, 201, 1),
            token!(String("Pointy McPointface".to_owned()), 203, 20),
            token!(Semicolon, 223, 1),
            token!(RightBrace, 237, 1),
            token!(RightBrace, 247, 1),
            token!(Eof, 257, 0),
        ];

        let tokens = tokenize(source);

        compare_tokens!(expected, tokens);
    }
}
